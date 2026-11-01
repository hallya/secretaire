//! Audio exchanged with providers.

/// Format of an audio payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
}

impl AudioFormat {
    /// MIME type, used in HTTP requests.
    pub fn mime_type(self) -> &'static str {
        match self {
            AudioFormat::Wav => "audio/wav",
            AudioFormat::Mp3 => "audio/mpeg",
        }
    }

    /// Usual file extension.
    pub fn extension(self) -> &'static str {
        match self {
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
        }
    }
}

/// Encoded audio payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audio {
    pub format: AudioFormat,
    pub bytes: Vec<u8>,
}
