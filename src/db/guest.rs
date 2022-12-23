use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::db::Group;
use crate::user::User;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Guest {
    pub(super) group: Group,
    #[serde(flatten)]
    pub(super) user: User,
    #[serde(flatten)]
    pub(super) rsvp: Option<Rsvp>,
}

impl Guest {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn update(&mut self, rsvp: Rsvp) {
        self.rsvp = Some(rsvp);
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reply {
    pub(super) attend: Attend,
    pub(super) meal: Option<Meal>,
    pub(super) msg: Option<String>,
}

impl From<Rsvp> for Reply {
    fn from(rsvp: Rsvp) -> Self {
        match rsvp {
            Rsvp::Yes { meal, msg } => Reply {
                attend: Attend::Yes,
                meal: Some(meal),
                msg: Some(msg),
            },
            Rsvp::No => Reply {
                attend: Attend::No,
                meal: None,
                msg: None,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
