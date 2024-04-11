use std::path::{Path, PathBuf};

use clap::Parser;
use rand::{self, Rng};
use rocket::{request::FromParam, UriDisplayPath};

use crate::user::User;

#[derive(UriDisplayPath)]
pub struct PasteId(String);

impl PasteId {
    pub fn new(size: usize) -> PasteId {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteId(id.to_string())
    }

    pub fn file_path(&self, owner_id: &Option<String>) -> PathBuf {
        let root = super::Args::parse().upload;
        let owner_id = if let Some(user_id) = owner_id {
            user_id
        } else {
            &User::anonymous().username
        };
        Path::new(&root).join(owner_id.to_string()).join(&self.0)
    }
}

impl<'a> FromParam<'a> for PasteId {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}
