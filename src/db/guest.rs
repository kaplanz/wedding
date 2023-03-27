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
    #[serde(default)]
    pub(super) child: bool,
    #[serde(flatten)]
    pub(super) reply: Reply,
}

impl Guest {
    pub fn group(&self) -> Group {
        self.group
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn child(&self) -> bool {
        self.child
    }

    pub fn reply(&self) -> &Reply {
        &self.reply
    }

    pub fn update(&mut self, reply: Reply) {
        self.reply = reply;
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Reply {
    pub attend: Option<Attend>,
    pub meal: Option<Meal>,
    pub msg: Option<Message>,
}

impl Reply {
    pub fn responded(&self) -> bool {
        self.attend.is_some() || self.meal.is_some() || self.msg.is_some()
    }

    pub fn validate(&mut self) {
        if !matches!(self.attend, Some(Attend::Yes)) {
            self.meal = None;
        }
    }
}

impl Display for Reply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(
            &[
                self.attend.as_ref().map(ToString::to_string),
                self.meal.as_ref().map(ToString::to_string),
                self.msg.as_ref().map(ToString::to_string),
            ]
            .iter()
            .flatten()
            .collect::<Vec<_>>(),
            f,
        )
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
    NoMeal,
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            match self {
                Meal::Chicken => "Chicken",
                Meal::Fish => "Fish",
                Meal::Veggie => "Vegetarian",
                Meal::Kids => "Kids Meal",
                Meal::NoMeal => "No Meal",
            },
            f,
        )
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
