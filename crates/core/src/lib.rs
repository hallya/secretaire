//! Secretaire core.
//!
//! Pure logic with no system access (files, audio, network), so that it
//! compiles both natively and to `wasm32` (decision D-008).

pub mod audio;
pub mod http;
pub mod provider;

pub use audio::{Audio, AudioFormat};
pub use http::{Body, HttpRequest, HttpResponse, Method, Part};
pub use provider::{Function, ProviderError, Registry, Selection, SpeechToText, TextToSpeech};

/// Library version, as declared in `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_cargo_toml() {
        assert_eq!(version(), "0.1.0");
    }
}
