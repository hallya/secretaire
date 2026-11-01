//! Cœur de Secretaire.
//!
//! Logique pure, sans accès au système (fichiers, audio, réseau), afin de
//! compiler à la fois en natif et en `wasm32` (décision D-008).

pub mod audio;
pub mod http;
pub mod provider;

pub use audio::{Audio, AudioFormat};
pub use http::{Body, HttpRequest, HttpResponse, Method, Part};
pub use provider::{Function, ProviderError, Registry, Selection, SpeechToText, TextToSpeech};

/// Version de la bibliothèque, telle que déclarée dans `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_correspond_au_cargo_toml() {
        assert_eq!(version(), "0.1.0");
    }
}
