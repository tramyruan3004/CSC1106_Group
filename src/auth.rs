use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid; // for uuid generation
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, 
    exp: usize,  
}

#[derive(Debug, Clone)]
pub struct Authenticated {
    pub user_id: String,
}

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token_opt = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|bearer| bearer.strip_prefix("Bearer "));

        if let Some(token) = token_opt {
            if let Ok(data) = decode::<Claims>(
                token,
                &DecodingKey::from_secret(b"secretkey"),
                &Validation::default(),
            ) {
                return ready(Ok(Authenticated {
                    user_id: data.claims.sub,
                }));
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}

pub fn create_token(user_id: Uuid) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap() 
        .timestamp() as usize; 

    let claims = Claims {
        sub: user_id.to_string(), 
        exp: expiration, 
    };

    encode(
        &Header::default(), // Use default JWT header settings.
        &claims,            // Pass in the claims data.
        &EncodingKey::from_secret(b"secretkey"), // Secret key for encoding.
    )
    .unwrap() 
}

pub fn validate_token(token: &str) -> bool {
    decode::<Claims>(
        token,                                 
        &DecodingKey::from_secret(b"secretkey"), 
        &Validation::default(),               
    )
    .is_ok() 
}