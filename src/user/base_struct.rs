use std::{fs, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::Args;

use super::UserToken;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub(super) password: String,
}

impl From<UserToken> for User {
    fn from(data: UserToken) -> Self {
        serde_json::from_str(
            fs::read_to_string(
                Path::new(&Args::parse().upload)
                    .join(data.username)
                    .join(".credentials")
                    .to_str()
                    .unwrap(),
            )
            .unwrap()
            .as_str(),
        )
        .unwrap()
    }
}

impl User {
    pub fn anonymous() -> Self {
        Self {
            username: String::from("anonymous"),
            password: String::new(),
        }
    }
}
