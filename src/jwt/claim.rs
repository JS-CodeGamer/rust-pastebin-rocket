use chrono::{serde::ts_seconds, DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use super::TokenError;

#[derive(Serialize, Deserialize)]
pub struct Claim<T> {
    #[serde(with = "ts_seconds")]
    exp: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    iat: DateTime<Utc>,
    data: T,
}

impl<T> Claim<T> {
    pub fn from_data(data: T) -> Result<Self, TokenError> {
        let jwt_valid_for = match std::env::var("JWT_EXPIRE_SECS") {
            Ok(x) => match x.parse() {
                Ok(x) => x,
                Err(_) => return Err(TokenError::JwtExpiryNotValid),
            },
            Err(_) => return Err(TokenError::JwtExpiryTimeNotFound),
        };
        Ok(Claim {
            iat: Utc::now(),
            exp: Utc::now() + TimeDelta::new(jwt_valid_for, 0).unwrap(),
            data,
        })
    }
    pub fn verify(self) -> Result<T, TokenError> {
        if self.exp < Utc::now() {
            Err(TokenError::TokenExpired)
        } else {
            Ok(self.data)
        }
    }
}
