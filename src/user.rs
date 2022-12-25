use std::fmt::Display;
use std::hash::Hash;

use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};

pub use crate::db::Ident;

#[derive(Clone, Debug, Default, Deserialize, Eq, Serialize)]
pub struct User {
    #[serde(skip)]
    pub(crate) ident: Ident,
    pub(super) first: String,
    pub(super) last: String,
}

impl User {
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

    pub fn sanitize(&mut self) {
        self.first = sanitize(&self.first);
        self.last = sanitize(&self.last);
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

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.last.hash(state);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first) && (self.last == other.last)
    }
}

fn sanitize(input: &str) -> String {
    let mut chars = input.trim().chars();
    chars
        .next()
        .into_iter()
        .flat_map(|c| c.to_uppercase())
        .chain(chars.flat_map(|c| c.to_lowercase()))
        .collect()
}
