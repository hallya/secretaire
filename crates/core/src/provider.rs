//! Fournisseurs de reconnaissance et de lecture, interchangeables (D-004).
//!
//! L'utilisateur choisit un fournisseur par son nom, séparément pour la
//! reconnaissance (audio → texte) et pour la lecture (texte → audio).

use crate::audio::Audio;
use crate::http::{HttpRequest, HttpResponse};
use std::fmt;

/// Reconnaissance vocale : audio → texte.
pub trait SpeechToText {
    /// Nom sous lequel l'utilisateur choisit ce fournisseur.
    fn name(&self) -> &str;
    /// Requête à envoyer pour transcrire `audio`.
    fn transcription_request(&self, audio: &Audio) -> HttpRequest;
    /// Lit le texte transcrit dans la réponse.
    fn parse_transcription(&self, response: &HttpResponse) -> Result<String, ProviderError>;
}

/// Lecture : texte → audio.
pub trait TextToSpeech {
    /// Nom sous lequel l'utilisateur choisit ce fournisseur.
    fn name(&self) -> &str;
    /// Requête à envoyer pour lire `text` à voix haute.
    fn synthesis_request(&self, text: &str) -> HttpRequest;
    /// Lit l'audio dans la réponse.
    fn parse_synthesis(&self, response: &HttpResponse) -> Result<Audio, ProviderError>;
}

/// Les deux fonctions de Secretaire.
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

/// Erreurs communes à tous les fournisseurs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    /// Aucun fournisseur de ce nom pour cette fonction.
    UnknownProvider { function: Function, name: String },
    /// Le fournisseur a répondu par une erreur HTTP.
    Http { status: u16, message: String },
    /// La réponse ne peut pas être lue.
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

/// Fournisseur choisi pour chaque fonction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub speech_to_text: String,
    pub text_to_speech: String,
}

/// Fournisseurs disponibles.
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

    /// Fournisseur de reconnaissance nommé `name`.
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

    /// Fournisseur de lecture nommé `name`.
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

    /// Un fournisseur par fonction, selon la sélection de l'utilisateur.
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

    /// Faux fournisseur de reconnaissance : renvoie le corps de la réponse comme texte.
    struct FakeSpeechToText(&'static str);

    impl SpeechToText for FakeSpeechToText {
        fn name(&self) -> &str {
            self.0
        }

        fn transcription_request(&self, audio: &Audio) -> HttpRequest {
            HttpRequest {
                method: Method::Post,
                url: format!("https://{}.test/transcrire", self.0),
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

    /// Faux fournisseur de lecture : renvoie le corps de la réponse comme audio WAV.
    struct FakeTextToSpeech(&'static str);

    impl TextToSpeech for FakeTextToSpeech {
        fn name(&self) -> &str {
            self.0
        }

        fn synthesis_request(&self, text: &str) -> HttpRequest {
            HttpRequest {
                method: Method::Post,
                url: format!("https://{}.test/lire", self.0),
                headers: vec![],
                body: Body::Json(format!("{{\"texte\":\"{text}\"}}")),
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
        registry.register_speech_to_text(Box::new(FakeSpeechToText("faux-a")));
        registry.register_speech_to_text(Box::new(FakeSpeechToText("faux-b")));
        registry.register_text_to_speech(Box::new(FakeTextToSpeech("faux-a")));
        registry.register_text_to_speech(Box::new(FakeTextToSpeech("faux-b")));
        registry
    }

    #[test]
    fn choisit_le_fournisseur_de_reconnaissance_par_son_nom() {
        let registry = registry();
        let provider = registry
            .speech_to_text("faux-b")
            .expect("fournisseur trouvé");
        assert_eq!(provider.name(), "faux-b");
    }

    #[test]
    fn choisit_independamment_reconnaissance_et_lecture() {
        let registry = registry();
        let selection = Selection {
            speech_to_text: "faux-a".into(),
            text_to_speech: "faux-b".into(),
        };
        let (stt, tts) = registry.resolve(&selection).expect("sélection valide");
        assert_eq!((stt.name(), tts.name()), ("faux-a", "faux-b"));
    }

    #[test]
    fn un_fournisseur_inconnu_donne_une_erreur_explicite() {
        let registry = registry();
        let error = registry.speech_to_text("inexistant").err().expect("erreur");
        assert_eq!(
            error,
            ProviderError::UnknownProvider {
                function: Function::SpeechToText,
                name: "inexistant".into(),
            }
        );
        assert_eq!(
            error.to_string(),
            "aucun fournisseur « inexistant » pour la reconnaissance"
        );
    }

    #[test]
    fn un_fournisseur_de_reconnaissance_fait_l_aller_retour_requete_reponse() {
        let registry = registry();
        let stt = registry
            .speech_to_text("faux-a")
            .expect("fournisseur trouvé");
        let audio = Audio {
            format: AudioFormat::Wav,
            bytes: vec![1, 2, 3],
        };

        let request = stt.transcription_request(&audio);
        assert_eq!(request.method, Method::Post);
        assert_eq!(request.url, "https://faux-a.test/transcrire");

        let response = HttpResponse {
            status: 200,
            body: b"bonjour".to_vec(),
        };
        assert!(response.is_success());
        assert_eq!(stt.parse_transcription(&response).unwrap(), "bonjour");
    }

    #[test]
    fn un_fournisseur_de_lecture_fait_l_aller_retour_requete_reponse() {
        let registry = registry();
        let tts = registry
            .text_to_speech("faux-b")
            .expect("fournisseur trouvé");

        let request = tts.synthesis_request("Bonjour");
        assert_eq!(request.url, "https://faux-b.test/lire");
        assert_eq!(request.body, Body::Json("{\"texte\":\"Bonjour\"}".into()));

        let response = HttpResponse {
            status: 200,
            body: vec![9, 9],
        };
        let audio = tts.parse_synthesis(&response).unwrap();
        assert_eq!(audio.format, AudioFormat::Wav);
        assert_eq!(audio.bytes, vec![9, 9]);
    }
}
