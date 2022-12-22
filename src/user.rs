use std::fmt::Display;

use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    first: String,
    last: String,
}

impl User {
    #[allow(unused)]
    pub fn new(first: String, last: String) -> Self {
        Self { first, last }
    }

    #[allow(unused)]
    pub fn first(&self) -> &str {
        self.first.as_ref()
    }

    #[allow(unused)]
    pub fn last(&self) -> &str {
        self.last.as_ref()
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}

impl AuthUser for User {
    fn get_id(&self) -> String {
        self.name()
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
