//! Requêtes et réponses HTTP, décrites sans être envoyées.
//!
//! Le `core` n'a pas accès au réseau (D-008) : il construit la requête et lit
//! la réponse ; c'est le programme du terminal (ou du navigateur) qui l'envoie.

/// Méthode HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

/// Corps d'une requête.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Empty,
    Json(String),
    Multipart(Vec<Part>),
}

/// Partie d'un formulaire `multipart/form-data`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Text {
        name: String,
        value: String,
    },
    File {
        name: String,
        filename: String,
        mime_type: String,
        bytes: Vec<u8>,
    },
}

/// Requête à envoyer à un fournisseur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Body,
}

/// Réponse reçue d'un fournisseur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Vrai pour un statut 2xx.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
