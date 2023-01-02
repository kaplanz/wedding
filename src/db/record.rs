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
        // Construct user
        let user = User {
            ident: Default::default(),
            first,
            last,
        };
        // Construct rsvp
        let rsvp = Reply::new(attend, meal, msg).into();
        // Construct guest
        Self {
            group,
            user,
            child,
            rsvp,
        }
    }
}

impl From<Guest> for Record {
    fn from(
        Guest {
            group,
            user,
            child,
            rsvp,
        }: Guest,
    ) -> Self {
        // Destructure first, last fields
        let User { first, last, .. } = user;
        // Destructure attend, meal, msg fields
        let (attend, meal, msg) = rsvp.map_or((None, None, None), |rsvp| {
            let Reply { attend, meal, msg } = rsvp.into();
            (attend, meal, msg)
        });
        // Construct record
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
