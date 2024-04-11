use std::marker::PhantomData;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
};
use serde::de::DeserializeOwned;
use sha2::Sha256;

use super::claim::Claim;
use super::TokenError;

#[derive(Serialize, Deserialize)]
pub struct Token<T: Serialize + DeserializeOwned>(String, PhantomData<T>);

#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for Token<T>
where
    T: Serialize + DeserializeOwned,
{
    type Error = TokenError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let token_str = if let Some(raw_header) = request.headers().get_one("Authorization") {
            if raw_header[..7] == *"Bearer " {
                raw_header[7..].to_string()
            } else {
                return Outcome::Error((Status::Unauthorized, Self::Error::MalformedAuthHeader));
            }
        } else {
            return Outcome::Error((Status::Unauthorized, Self::Error::NoAuthHeader));
        };
        Outcome::Success(Token::<T>(token_str, PhantomData))
    }
}

impl<T> Token<T>
where
    T: Serialize + DeserializeOwned,
{
    fn get_key() -> Result<Hmac<Sha256>, TokenError> {
        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(s) => s,
            Err(_) => return Err(TokenError::SecretNotFound),
        };
        match Hmac::new_from_slice(jwt_secret.as_bytes()) {
            Ok(k) => Ok(k),
            Err(e) => Err(TokenError::KeyCreationError(e)),
        }
    }

    pub fn encode(data: T) -> Result<Self, TokenError> {
        let key = match Self::get_key() {
            Ok(k) => k,
            Err(e) => return Err(e),
        };
        let claim = Claim::<T>::from_data(data)?;
        let token_str = match claim.sign_with_key(&key) {
            Ok(t) => t,
            Err(_) => return Err(TokenError::JwtError),
        };
        Ok(Token::<T>(token_str, PhantomData))
    }

    pub fn decode(self) -> Result<T, TokenError> {
        let key = Self::get_key()?;
        let claim: Claim<T> = match self.0.verify_with_key(&key) {
            Ok(t) => t,
            Err(_) => return Err(TokenError::JwtError),
        };

        let data = claim.verify()?;
        Ok(data)
    }
}
