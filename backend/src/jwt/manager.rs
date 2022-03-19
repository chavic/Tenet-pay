use crate::errors::ServiceError;
use crate::jwt::model::Claims;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub fn create_token(
    user_id: String,
    phone_number: String,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user_id, phone_number, auth_duration_in_hour);

    let encoding_key =
        EncodingKey::from_rsa_pem(include_bytes!("../../keys/rs256-4096-private.rsa"))
            .map_err(|e| ServiceError::InternalServerError(format!("{}", e)))?;

    encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    let decoding_key =
        DecodingKey::from_rsa_pem(include_bytes!("../../keys/rs256-4096-public.pem"))
            .map_err(|e| ServiceError::InternalServerError(format!("{}", e)))?;

    decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::RS256))
        .map(|data| data.claims)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
