use std::fmt::{Debug, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::db::Group;
use crate::user::User;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Guest {
    pub(super) group: Group,
    #[serde(flatten)]
    pub(super) user: User,
    pub(super) child: bool,
    #[serde(flatten)]
    pub(super) rsvp: Option<Rsvp>,
}

impl Guest {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn child(&self) -> bool {
        self.child
    }

    pub fn msg(&self) -> Option<Message> {
        self.reply()?.msg
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
    Yes { meal: Meal, msg: Message },
    No { msg: Message },
}

impl Display for Rsvp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &match self {
                Self::Yes { meal, msg } => format!("yes (meal: {meal:?}, msg: \"{msg}\")"),
                Self::No { msg } => format!("no (msg: \"{msg}\")"),
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
            Some(Attend::No) => Some(Rsvp::No {
                msg: reply.msg.unwrap_or_default(),
            }),
            None => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reply {
    #[serde(default)]
    pub attend: Option<Attend>,
    pub meal: Option<Meal>,
    pub msg: Option<Message>,
}

impl Reply {
    pub fn new(attend: Option<Attend>, meal: Option<Meal>, msg: Option<Message>) -> Self {
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
            Rsvp::No { msg } => Reply {
                attend: Some(Attend::No),
                meal: None,
                msg: Some(msg),
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
    Kids,
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Message(String);

impl Deref for Message {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
