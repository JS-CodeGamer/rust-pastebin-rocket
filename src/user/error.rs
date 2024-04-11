use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde_json::json;

#[derive(Debug, strum_macros::AsRefStr)]
pub enum Error {
    UserExists,
    IncorrectCredentials,
    PasswordsDontMatch,
    RegistrationFailed,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let code = self.as_ref();
        let (status, json_res) = match self {
            Self::RegistrationFailed => (
                Status::InternalServerError,
                json!({
                    "error" : "registration failed, if problem persists contact admins",
                    "code" : code
                }),
            ),
            Self::IncorrectCredentials => (
                Status::BadRequest,
                json!({
                    "error" : "please provide correct credentials",
                    "code" : code
                }),
            ),
            Self::UserExists => (
                Status::BadRequest,
                json!({
                    "error" : "failed to register, username already exists",
                    "code" : code
                }),
            ),
            Self::PasswordsDontMatch => (
                Status::Unauthorized,
                json!({
                    "error" : "failed to register, passwords dont match",
                    "code" : code
                }),
            ),
        };
        Response::build_from(json_res.respond_to(req)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}
