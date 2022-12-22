use std::fmt::Display;

use serde::Serialize;

use crate::user::User;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Guest {
    #[serde(flatten)]
    pub user: User,
    rsvp: Option<Rsvp>,
}

impl Guest {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn rsvp(&mut self, rsvp: Rsvp) {
        self.rsvp = Some(rsvp);
    }
}

impl From<User> for Guest {
    fn from(user: User) -> Self {
        Guest {
            user,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Rsvp {
    Yes { meal: Meal, msg: String },
    No,
}

impl Display for Rsvp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes { meal, msg } => format!("yes(meal: {meal:?}, msg: \"{msg}\")"),
            Self::No => "no".to_string(),
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Meal {
    Meat,
    Fish,
    Veggie,
}
