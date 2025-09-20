use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Middleware JWT pour sécuriser les routes
pub async fn require_auth(
    mut req: Request<axum::body::Body>,
    next: Next, // <-- plus de generic
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    if let Some(header_value) = auth_header {
        if let Some(token) = header_value.strip_prefix("Bearer ") {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "supersecretkeychangeit".to_string());
            let validation = Validation::default();
            let key = DecodingKey::from_secret(secret.as_bytes());

            match decode::<Claims>(token, &key, &validation) {
                Ok(data) => {
                    info!("Utilisateur authentifié: {}", data.claims.sub);
                    req.extensions_mut().insert(data.claims);
                    return Ok(next.run(req).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
