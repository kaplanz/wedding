use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Guest {
    #[serde(flatten)]
    pub user: User,
    rsvp: Option<Rsvp>,
}

impl Guest {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn update(&mut self, rsvp: Rsvp) {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl From<Reply> for Rsvp {
    fn from(reply: Reply) -> Self {
        match reply.attend {
            Attend::Yes => Self::Yes {
                meal: reply.meal.unwrap_or_default(),
                msg: reply.msg.unwrap_or_default(),
            },
            Attend::No => Self::No,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reply {
    attend: Attend,
    meal: Option<Meal>,
    msg: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Attend {
    Yes,
    No,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Meal {
    #[default]
    Meat,
    Fish,
    Veggie,
}
