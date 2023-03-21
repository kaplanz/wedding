use serde::{Deserialize, Serialize};

use super::guest::{Attend, Guest, Meal, Message, Reply};
use super::Group;
use crate::user::User;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Record {
    group: Group,
    first: String,
    last: String,
    #[serde(default)]
    child: bool,
    attend: Option<Attend>,
    meal: Option<Meal>,
    msg: Option<Message>,
}

impl From<Record> for Guest {
    fn from(
        Record {
            group,
            first,
            last,
            child,
            attend,
            meal,
            msg,
        }: Record,
    ) -> Self {
        Self {
            group,
            user: User::new(first, last),
            child,
            reply: Reply { attend, meal, msg },
        }
    }
}

impl From<Guest> for Record {
    fn from(
        Guest {
            group,
            user,
            child,
            reply,
        }: Guest,
    ) -> Self {
        let User { first, last, .. } = user;
        let Reply { attend, meal, msg } = reply;
        Self {
            group,
            first,
            last,
            child,
            attend,
            meal,
            msg,
        }
    }
}
