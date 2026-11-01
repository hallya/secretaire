//! Interchangeable speech-to-text and text-to-speech providers (D-004).
//!
//! The user picks a provider by name, separately for speech-to-text
//! (audio → text) and text-to-speech (text → audio).

use crate::audio::Audio;
use crate::http::{HttpRequest, HttpResponse};
use std::fmt;

/// Speech-to-text: audio → text.
pub trait SpeechToText {
    /// Name under which the user selects this provider.
    fn name(&self) -> &str;
    /// Request to send in order to transcribe `audio`.
    fn transcription_request(&self, audio: &Audio) -> HttpRequest;
    /// Extracts the transcribed text from the response.
    fn parse_transcription(&self, response: &HttpResponse) -> Result<String, ProviderError>;
}

/// Text-to-speech: text → audio.
pub trait TextToSpeech {
    /// Name under which the user selects this provider.
    fn name(&self) -> &str;
    /// Request to send in order to speak `text` aloud.
    fn synthesis_request(&self, text: &str) -> HttpRequest;
    /// Extracts the audio from the response.
    fn parse_synthesis(&self, response: &HttpResponse) -> Result<Audio, ProviderError>;
}

/// The two functions of Secretaire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    SpeechToText,
    TextToSpeech,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Function::SpeechToText => "reconnaissance",
            Function::TextToSpeech => "lecture",
        })
    }
}

/// Errors shared by all providers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    /// No provider with this name for this function.
    UnknownProvider { function: Function, name: String },
    /// The provider answered with an HTTP error.
    Http { status: u16, message: String },
    /// The response cannot be parsed.
    InvalidResponse(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::UnknownProvider { function, name } => {
                write!(f, "aucun fournisseur « {name} » pour la {function}")
            }
            ProviderError::Http { status, message } => {
                write!(f, "le fournisseur a répondu {status} : {message}")
            }
            ProviderError::InvalidResponse(detail) => write!(f, "réponse illisible : {detail}"),
        }
    }
}

impl std::error::Error for ProviderError {}

/// Provider chosen for each function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub speech_to_text: String,
    pub text_to_speech: String,
}

/// Available providers.
#[derive(Default)]
pub struct Registry {
    speech_to_text: Vec<Box<dyn SpeechToText>>,
    text_to_speech: Vec<Box<dyn TextToSpeech>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_speech_to_text(&mut self, provider: Box<dyn SpeechToText>) {
        self.speech_to_text.push(provider);
    }

    pub fn register_text_to_speech(&mut self, provider: Box<dyn TextToSpeech>) {
        self.text_to_speech.push(provider);
    }

    /// Speech-to-text provider named `name`.
    pub fn speech_to_text(&self, name: &str) -> Result<&dyn SpeechToText, ProviderError> {
        self.speech_to_text
            .iter()
            .find(|provider| provider.name() == name)
            .map(|provider| provider.as_ref())
            .ok_or_else(|| ProviderError::UnknownProvider {
                function: Function::SpeechToText,
                name: name.to_owned(),
            })
    }

    /// Text-to-speech provider named `name`.
    pub fn text_to_speech(&self, name: &str) -> Result<&dyn TextToSpeech, ProviderError> {
        self.text_to_speech
            .iter()
            .find(|provider| provider.name() == name)
            .map(|provider| provider.as_ref())
            .ok_or_else(|| ProviderError::UnknownProvider {
                function: Function::TextToSpeech,
                name: name.to_owned(),
            })
    }

    /// One provider per function, according to the user's selection.
    pub fn resolve(
        &self,
        selection: &Selection,
    ) -> Result<(&dyn SpeechToText, &dyn TextToSpeech), ProviderError> {
        Ok((
            self.speech_to_text(&selection.speech_to_text)?,
            self.text_to_speech(&selection.text_to_speech)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::AudioFormat;
    use crate::http::{Body, Method, Part};

    /// Fake speech-to-text provider: returns the response body as text.
    struct FakeSpeechToText(&'static str);

    impl SpeechToText for FakeSpeechToText {
        fn name(&self) -> &str {
            self.0
        }

        fn transcription_request(&self, audio: &Audio) -> HttpRequest {
            HttpRequest {
                method: Method::Post,
                url: format!("https://{}.test/transcribe", self.0),
                headers: vec![],
                body: Body::Multipart(vec![Part::File {
                    name: "file".into(),
                    filename: format!("audio.{}", audio.format.extension()),
                    mime_type: audio.format.mime_type().into(),
                    bytes: audio.bytes.clone(),
                }]),
            }
        }

        fn parse_transcription(&self, response: &HttpResponse) -> Result<String, ProviderError> {
            String::from_utf8(response.body.clone())
                .map_err(|e| ProviderError::InvalidResponse(e.to_string()))
        }
    }

    /// Fake text-to-speech provider: returns the response body as WAV audio.
    struct FakeTextToSpeech(&'static str);

    impl TextToSpeech for FakeTextToSpeech {
        fn name(&self) -> &str {
            self.0
        }

        fn synthesis_request(&self, text: &str) -> HttpRequest {
            HttpRequest {
                method: Method::Post,
                url: format!("https://{}.test/speak", self.0),
                headers: vec![],
                body: Body::Json(format!("{{\"text\":\"{text}\"}}")),
            }
        }

        fn parse_synthesis(&self, response: &HttpResponse) -> Result<Audio, ProviderError> {
            Ok(Audio {
                format: AudioFormat::Wav,
                bytes: response.body.clone(),
            })
        }
    }

    fn registry() -> Registry {
        let mut registry = Registry::new();
        registry.register_speech_to_text(Box::new(FakeSpeechToText("fake-a")));
        registry.register_speech_to_text(Box::new(FakeSpeechToText("fake-b")));
        registry.register_text_to_speech(Box::new(FakeTextToSpeech("fake-a")));
        registry.register_text_to_speech(Box::new(FakeTextToSpeech("fake-b")));
        registry
    }

    #[test]
    fn selects_speech_to_text_provider_by_name() {
        let registry = registry();
        let provider = registry.speech_to_text("fake-b").expect("provider found");
        assert_eq!(provider.name(), "fake-b");
    }

    #[test]
    fn selects_speech_to_text_and_text_to_speech_independently() {
        let registry = registry();
        let selection = Selection {
            speech_to_text: "fake-a".into(),
            text_to_speech: "fake-b".into(),
        };
        let (stt, tts) = registry.resolve(&selection).expect("valid selection");
        assert_eq!((stt.name(), tts.name()), ("fake-a", "fake-b"));
    }

    #[test]
    fn unknown_provider_returns_explicit_error() {
        let registry = registry();
        let error = registry.speech_to_text("missing").err().expect("error");
        assert_eq!(
            error,
            ProviderError::UnknownProvider {
                function: Function::SpeechToText,
                name: "missing".into(),
            }
        );
        assert_eq!(
            error.to_string(),
            "no speech-to-text provider named \"missing\""
        );
    }

    #[test]
    fn provider_errors_have_english_messages() {
        let http = ProviderError::Http {
            status: 401,
            message: "Unauthorized".into(),
        };
        assert_eq!(
            http.to_string(),
            "provider responded with HTTP 401: Unauthorized"
        );
        let invalid = ProviderError::InvalidResponse("missing field `text`".into());
        assert_eq!(
            invalid.to_string(),
            "invalid provider response: missing field `text`"
        );
        let unknown = ProviderError::UnknownProvider {
            function: Function::TextToSpeech,
            name: "missing".into(),
        };
        assert_eq!(
            unknown.to_string(),
            "no text-to-speech provider named \"missing\""
        );
    }

    #[test]
    fn speech_to_text_provider_round_trips_request_and_response() {
        let registry = registry();
        let stt = registry.speech_to_text("fake-a").expect("provider found");
        let audio = Audio {
            format: AudioFormat::Wav,
            bytes: vec![1, 2, 3],
        };

        let request = stt.transcription_request(&audio);
        assert_eq!(request.method, Method::Post);
        assert_eq!(request.url, "https://fake-a.test/transcribe");

        let response = HttpResponse {
            status: 200,
            body: b"hello".to_vec(),
        };
        assert!(response.is_success());
        assert_eq!(stt.parse_transcription(&response).unwrap(), "hello");
    }

    #[test]
    fn text_to_speech_provider_round_trips_request_and_response() {
        let registry = registry();
        let tts = registry.text_to_speech("fake-b").expect("provider found");

        let request = tts.synthesis_request("Hello");
        assert_eq!(request.url, "https://fake-b.test/speak");
        assert_eq!(request.body, Body::Json("{\"text\":\"Hello\"}".into()));

        let response = HttpResponse {
            status: 200,
            body: vec![9, 9],
        };
        let audio = tts.parse_synthesis(&response).unwrap();
        assert_eq!(audio.format, AudioFormat::Wav);
        assert_eq!(audio.bytes, vec![9, 9]);
    }
}
