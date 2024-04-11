use serde::{Deserialize, Serialize};

use super::User;

#[derive(Serialize, Deserialize)]
pub struct TokenStruct {
    pub(super) username: String,
}

impl From<User> for TokenStruct {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
        }
    }
}
