//! Audio échangé avec les fournisseurs.

/// Format d'un contenu audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
}

impl AudioFormat {
    /// Type MIME, utilisé dans les requêtes HTTP.
    pub fn mime_type(self) -> &'static str {
        match self {
            AudioFormat::Wav => "audio/wav",
            AudioFormat::Mp3 => "audio/mpeg",
        }
    }

    /// Extension de fichier habituelle.
    pub fn extension(self) -> &'static str {
        match self {
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
        }
    }
}

/// Contenu audio encodé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audio {
    pub format: AudioFormat,
    pub bytes: Vec<u8>,
}
