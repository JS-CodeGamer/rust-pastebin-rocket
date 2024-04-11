use std::io::Error as IOError;

use rocket::{
    response::{self, Responder},
    Request,
};

use crate::{jwt::TokenError, user::UserError};

#[derive(Debug, strum_macros::AsRefStr)]
pub enum Error {
    TokenError(TokenError),
    IOError(IOError),
    UserError(UserError),
}

impl From<TokenError> for Error {
    fn from(err: TokenError) -> Self {
        Self::TokenError(err)
    }
}
impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}
impl From<UserError> for Error {
    fn from(err: UserError) -> Self {
        Self::UserError(err)
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::TokenError(x) => x.respond_to(req),
            Self::IOError(x) => x.respond_to(req),
            Self::UserError(x) => x.respond_to(req),
        }
    }
}
