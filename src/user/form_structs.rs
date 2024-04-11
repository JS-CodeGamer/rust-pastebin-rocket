use std::{fs, path::Path};

use clap::Parser;
use serde::Deserialize;

use crate::Args;

use super::{User, UserError};

#[derive(Deserialize)]
pub struct RegUserForm {
    password: String,
    confirm_password: String,
    username: String,
}

impl User {
    pub fn register(form_data: RegUserForm) -> Result<(), UserError> {
        if form_data.password != form_data.confirm_password {
            return Err(UserError::PasswordsDontMatch);
        }
        let new_user = Self {
            username: form_data.username,
            password: form_data.password,
        };
        let cred_string = serde_json::to_string(&new_user).unwrap();
        let root = Args::parse().upload;
        let user_dir = Path::new(&root).join(new_user.username);
        if user_dir.exists() {
            return Err(UserError::UserExists);
        } else {
            if let Err(_) = fs::create_dir(&user_dir) {
                return Err(UserError::RegistrationFailed);
            };
        }
        if let Err(_) = fs::write(user_dir.join(".credentials").to_str().unwrap(), cred_string) {
            let _ = fs::remove_dir(&user_dir);
            return Err(UserError::RegistrationFailed);
        };
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct PassWordChangeForm {
    old_password: String,
    new_password: String,
}
impl User {
    pub fn change_password(&mut self, form_data: PassWordChangeForm) -> Result<(), UserError> {
        if form_data.old_password == self.password {
            self.password = form_data.new_password;
            Ok(())
        } else {
            Err(UserError::IncorrectCredentials)
        }
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}
impl User {
    pub fn login(form_data: LoginForm) -> Result<Self, UserError> {
        let root = Args::parse().upload;
        let creds = if let Ok(c) = fs::read_to_string(
            Path::new(&root)
                .join(form_data.username)
                .join(".credentials")
                .to_str()
                .unwrap(),
        ) {
            c
        } else {
            return Err(UserError::IncorrectCredentials);
        };
        let user: User = if let Ok(u) = serde_json::from_str(creds.as_str()) {
            u
        } else {
            return Err(UserError::IncorrectCredentials);
        };
        if user.password == form_data.password {
            Ok(user)
        } else {
            Err(UserError::IncorrectCredentials)
        }
    }
}
