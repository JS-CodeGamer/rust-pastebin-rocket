use hmac::digest::InvalidLength;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde_json::json;

#[derive(Debug, strum_macros::AsRefStr)]
pub enum Error {
    JwtError,
    SecretNotFound,
    JwtExpiryTimeNotFound,
    JwtExpiryNotValid,
    KeyCreationError(InvalidLength),
    TokenExpired,
    MalformedAuthHeader,
    NoAuthHeader,
}

impl Error {
    pub fn to_response_params(self) -> (Status, serde_json::Value) {
        let code = self.as_ref();
        match self {
            Self::NoAuthHeader => (
                Status::BadRequest,
                json!({
                    "error" : "please provide credentials",
                    "code" : code
                }),
            ),
            Self::TokenExpired | Self::JwtError => (
                Status::Unauthorized,
                json!({
                    "error" : "invalid token",
                    "code" : code
                }),
            ),
            Self::JwtExpiryTimeNotFound
            | Self::JwtExpiryNotValid
            | Self::SecretNotFound
            | Self::MalformedAuthHeader
            | Self::KeyCreationError(_) => (
                Status::InternalServerError,
                json!({
                    "error" : "please contact admins",
                    "code" : code
                }),
            ),
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let (status, json_res) = self.to_response_params();
        Response::build_from(json_res.respond_to(req)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}
