use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Ressource non trouvée")]
    NotFound,
    #[error("Accès non autorisé")]
    Unauthorized,
    #[error("Erreur interne: {0}")]
    Internal(String),

    #[error("Requête invalide: {0}")]
    BadRequest(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Erreur de hachage du mot de passe")]
    PasswordHashingError, // Simpler variant without the original error
}

// Manual implementation of From for argon2::password_hash::Error
impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        // We ignore the specific error and return a generic variant.
        // This avoids the as_dyn_error issue.
        AppError::PasswordHashingError
    }
}

// Implement IntoResponse as before
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // Ces erreurs sont gérées directement avec un message statique.
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            // L'erreur interne contient déjà le message, on le retourne directement.
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            // Pour les erreurs de la base de données (SqlxError),
            // on utilise le message de l'erreur sous-jacente.
            AppError::SqlxError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            // Pour l'erreur de hachage, on peut soit retourner un message générique pour des raisons de sécurité,
            // soit un message plus spécifique.
            AppError::PasswordHashingError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors du hachage du mot de passe".to_string()).into_response()
            }
        }
    }
}