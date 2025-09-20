use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};


#[derive(Serialize, Deserialize)]
pub struct Claims {
pub sub: String,
pub exp: usize,
}


pub fn generate_token(user_id: &str, secret: &str) -> String {
let expiration = Utc::now() + Duration::hours(24);
let claims = Claims {
sub: user_id.to_string(),
exp: expiration.timestamp() as usize,
};
encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}