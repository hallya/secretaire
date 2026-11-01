//! Mistral provider (Voxtral): builds requests and parses responses (D-007, D-009).
//!
//! Transcription API: `POST /v1/audio/transcriptions`, multipart form with
//! `model`, `file` and an optional `language`; the JSON response holds `text`.

use std::fmt;

use crate::audio::Audio;
use crate::http::{Body, HttpRequest, HttpResponse, Method, Part};
use crate::provider::{ProviderError, SpeechToText};

/// Name under which the Mistral provider is selected.
pub const PROVIDER_NAME: &str = "mistral";
/// Public Mistral API base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.mistral.ai";
/// Default transcription model (Voxtral Mini Transcribe).
pub const DEFAULT_TRANSCRIPTION_MODEL: &str = "voxtral-mini-transcribe-3";

/// Mistral speech-to-text (Voxtral transcription).
#[derive(Clone, PartialEq, Eq)]
pub struct MistralSpeechToText {
    api_key: String,
    base_url: String,
    model: String,
    language: Option<String>,
}

impl MistralSpeechToText {
    /// Provider using the default model and the public Mistral API.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_owned(),
            model: DEFAULT_TRANSCRIPTION_MODEL.to_owned(),
            language: None,
        }
    }

    /// Uses another transcription model.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Pins the audio language (for example `fr`), which can improve accuracy.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sends requests to another base URL (proxy, test server).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

/// Never prints the API key.
impl fmt::Debug for MistralSpeechToText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MistralSpeechToText")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("language", &self.language)
            .finish()
    }
}

impl SpeechToText for MistralSpeechToText {
    fn name(&self) -> &str {
        PROVIDER_NAME
    }

    fn transcription_request(&self, _audio: &Audio) -> HttpRequest {
        // Red step of T11: not implemented yet.
        HttpRequest {
            method: Method::Post,
            url: String::new(),
            headers: vec![],
            body: Body::Empty,
        }
    }

    fn parse_transcription(&self, _response: &HttpResponse) -> Result<String, ProviderError> {
        // Red step of T11: not implemented yet.
        Err(ProviderError::InvalidResponse("not implemented".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::AudioFormat;
    use crate::provider::Registry;

    /// Response shape from the Mistral API reference (2026-11-01).
    /// T12 checks it against the live API.
    const DOCUMENTED_RESPONSE: &str =
        include_str!("../fixtures/mistral/transcription_response.json");

    fn wav() -> Audio {
        Audio {
            format: AudioFormat::Wav,
            bytes: vec![0x52, 0x49, 0x46, 0x46],
        }
    }

    fn response(status: u16, body: &str) -> HttpResponse {
        HttpResponse {
            status,
            body: body.as_bytes().to_vec(),
        }
    }

    fn multipart_parts(request: HttpRequest) -> Vec<Part> {
        match request.body {
            Body::Multipart(parts) => parts,
            other => panic!("multipart body expected, got {other:?}"),
        }
    }

    #[test]
    fn transcription_request_targets_the_mistral_endpoint_with_a_bearer_key() {
        let request = MistralSpeechToText::new("test-key").transcription_request(&wav());
        assert_eq!(request.method, Method::Post);
        assert_eq!(
            request.url,
            "https://api.mistral.ai/v1/audio/transcriptions"
        );
        assert_eq!(
            request.headers,
            vec![("Authorization".to_owned(), "Bearer test-key".to_owned())]
        );
    }

    #[test]
    fn transcription_request_sends_the_model_and_the_audio_file_as_multipart() {
        let request = MistralSpeechToText::new("test-key").transcription_request(&wav());
        assert_eq!(
            multipart_parts(request),
            vec![
                Part::Text {
                    name: "model".into(),
                    value: "voxtral-mini-transcribe-3".into(),
                },
                Part::File {
                    name: "file".into(),
                    filename: "audio.wav".into(),
                    mime_type: "audio/wav".into(),
                    bytes: vec![0x52, 0x49, 0x46, 0x46],
                },
            ]
        );
    }

    #[test]
    fn transcription_request_includes_the_language_only_when_set() {
        let language = Part::Text {
            name: "language".into(),
            value: "fr".into(),
        };
        let with_language = MistralSpeechToText::new("k")
            .with_language("fr")
            .transcription_request(&wav());
        assert!(multipart_parts(with_language).contains(&language));

        let without_language = MistralSpeechToText::new("k").transcription_request(&wav());
        assert!(
            !multipart_parts(without_language)
                .iter()
                .any(|part| matches!(part, Part::Text { name, .. } if name == "language"))
        );
    }

    #[test]
    fn custom_model_and_base_url_are_used() {
        let request = MistralSpeechToText::new("k")
            .with_model("voxtral-small-transcribe-3")
            .with_base_url("http://localhost:8080/")
            .transcription_request(&wav());
        assert_eq!(request.url, "http://localhost:8080/v1/audio/transcriptions");
        assert_eq!(
            multipart_parts(request)[0],
            Part::Text {
                name: "model".into(),
                value: "voxtral-small-transcribe-3".into(),
            }
        );
    }

    #[test]
    fn parses_the_text_of_the_documented_response() {
        let text = MistralSpeechToText::new("k")
            .parse_transcription(&response(200, DOCUMENTED_RESPONSE))
            .expect("text parsed");
        assert_eq!(text, "Hello, this is a transcription test.");
    }

    #[test]
    fn http_errors_carry_the_provider_message() {
        let stt = MistralSpeechToText::new("k");
        assert_eq!(
            stt.parse_transcription(&response(401, r#"{"detail":"Invalid API Key"}"#)),
            Err(ProviderError::Http {
                status: 401,
                message: "Invalid API Key".into(),
            })
        );
        assert_eq!(
            stt.parse_transcription(&response(
                400,
                r#"{"object":"error","message":"Invalid model","type":"invalid_request_error"}"#
            )),
            Err(ProviderError::Http {
                status: 400,
                message: "Invalid model".into(),
            })
        );
        assert_eq!(
            stt.parse_transcription(&response(502, "Bad Gateway")),
            Err(ProviderError::Http {
                status: 502,
                message: "Bad Gateway".into(),
            })
        );
    }

    #[test]
    fn unreadable_responses_are_reported_as_invalid() {
        let stt = MistralSpeechToText::new("k");
        assert!(matches!(
            stt.parse_transcription(&response(200, "not json")),
            Err(ProviderError::InvalidResponse(_))
        ));
        assert_eq!(
            stt.parse_transcription(&response(200, r#"{"model":"voxtral-mini-transcribe-3"}"#)),
            Err(ProviderError::InvalidResponse(
                "missing field `text`".into()
            ))
        );
    }

    #[test]
    fn debug_output_never_contains_the_api_key() {
        let debug = format!("{:?}", MistralSpeechToText::new("secret-key"));
        assert!(!debug.contains("secret-key"));
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn can_be_selected_by_name_in_the_registry() {
        let mut registry = Registry::new();
        registry.register_speech_to_text(Box::new(MistralSpeechToText::new("k")));
        let provider = registry.speech_to_text("mistral").expect("provider found");
        assert_eq!(provider.name(), "mistral");
    }
}
