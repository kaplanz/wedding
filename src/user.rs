use std::fmt::Display;

use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct User {
    first: String,
    last: String,
}

impl User {
    #[allow(unused)]
    pub fn new(first: String, last: String) -> Self {
        Self { first, last }
    }
}

impl AuthUser for User {
    fn get_id(&self) -> String {
        format!("{} {}", self.first, self.last)
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(Default::default())
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_id().fmt(f)
    }
}
