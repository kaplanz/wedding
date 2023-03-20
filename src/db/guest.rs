use std::fmt::{Debug, Display};
use std::ops::Deref;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use crate::db::Group;
use crate::user::User;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Guest {
    group: Group,
    #[serde(flatten)]
    user: User,
    #[serde(default)]
    child: bool,
    #[serde(flatten)]
    reply: Option<Reply>,
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

    pub fn reply(&self) -> Option<&Reply> {
        self.reply.as_ref()
    }

    pub fn msg(&self) -> Option<Message> {
        self.reply.clone()?.msg
    }

    pub fn update(&mut self, reply: Reply) {
        self.reply = Some(reply);
    }
}

impl Serialize for Guest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut row = serializer.serialize_struct("Guest", 7)?;
        row.serialize_field("group", &self.group)?;
        row.serialize_field("first", &self.user.first)?;
        row.serialize_field("last", &self.user.last)?;
        row.serialize_field("child", &self.child)?;
        row.serialize_field("attend", &self.reply.as_ref().map(|reply| &reply.attend))?;
        row.serialize_field("meal", &self.reply.as_ref().map(|reply| &reply.meal))?;
        row.serialize_field("msg", &self.reply.as_ref().map(|reply| &reply.msg))?;
        row.end()
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
    pub fn validate(&mut self) {
        if !matches!(self.attend, Some(Attend::Yes)) {
            self.meal = None;
        }
    }
}

impl Display for Reply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.attend {
            Some(attend) => Display::fmt(&attend, f),
            None => Ok(()),
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
