use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::Result as JWTResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    /// user id
    sub: i32,
    /// issued at (in UTC timestamp)
    iat: i64,
    /// expire at (in UTC timestamp). Validated
    pub(self) exp: i64,
}

impl JWT {
    pub fn new(user_id: i32, duration: i64) -> Self {
        let now = Utc::now();
        let ts = now.timestamp();
        JWT {
            sub: user_id,
            iat: ts,
            exp: ts + duration,
        }
    }

    pub fn to_token(&self, secret: &str) -> JWTResult<String> {
        encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_base64_secret("amlhaHVpYmlzaGU=")?,
        )
    }

    pub fn from_token(token: &str, secret: &str) -> JWTResult<Self> {
        decode::<JWT>(
            token,
            &DecodingKey::from_base64_secret("amlhaHVpYmlzaGU=")?,
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn user_id(&self) -> i32 {
        self.sub
    }

    pub fn issued_at(&self) -> i64 {
        self.iat
    }
}