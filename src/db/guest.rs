use std::fmt::{Debug, Display};

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

    pub fn reply(&self) -> Option<Reply> {
        self.rsvp.clone().map(Reply::from)
    }

    pub fn update(&mut self, rsvp: Option<Rsvp>) {
        self.rsvp = rsvp;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Rsvp {
    Yes { meal: Meal, msg: String },
    No,
}

impl Display for Rsvp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &match self {
                Self::Yes { meal, msg } => format!("yes (meal: {meal:?}, msg: \"{msg}\")"),
                Self::No => "no".to_string(),
            },
            f,
        )
    }
}

impl From<Reply> for Option<Rsvp> {
    fn from(reply: Reply) -> Self {
        match reply.attend {
            Some(Attend::Yes) => Some(Rsvp::Yes {
                meal: reply.meal.unwrap_or_default(),
                msg: reply.msg.unwrap_or_default(),
            }),
            Some(Attend::No) => Some(Rsvp::No),
            None => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reply {
    #[serde(default)]
    pub attend: Option<Attend>,
    pub meal: Option<Meal>,
    pub msg: Option<String>,
}

impl Reply {
    pub fn new(attend: Option<Attend>, meal: Option<Meal>, msg: Option<String>) -> Self {
        Self { attend, meal, msg }
    }
}

impl From<Rsvp> for Reply {
    fn from(rsvp: Rsvp) -> Self {
        match rsvp {
            Rsvp::Yes { meal, msg } => Reply {
                attend: Some(Attend::Yes),
                meal: Some(meal),
                msg: Some(msg),
            },
            Rsvp::No => Reply {
                attend: Some(Attend::No),
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

impl Display for Attend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Meal {
    #[default]
    Chicken,
    Fish,
    Veggie,
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
