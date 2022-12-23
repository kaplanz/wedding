use serde::{Deserialize, Serialize};

use super::guest::{Attend, Guest, Meal, Reply, Rsvp};
use super::Group;
use crate::user::User;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Record {
    group: Group,
    first: String,
    last: String,
    attend: Option<Attend>,
    meal: Option<Meal>,
    msg: Option<String>,
}

impl From<Record> for Guest {
    fn from(
        Record {
            group,
            first,
            last,
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
        let rsvp = attend.map(|attend| Rsvp::from(Reply { attend, meal, msg }));
        // Construct guest
        Self { group, user, rsvp }
    }
}

impl From<Guest> for Record {
    fn from(Guest { group, user, rsvp }: Guest) -> Self {
        // Destructure first, last fields
        let User { first, last, .. } = user;
        // Destructure attend, meal, msg fields
        let (attend, meal, msg) = rsvp.map_or((None, None, None), |rsvp| {
            let Reply { attend, meal, msg } = rsvp.into();
            (Some(attend), meal, msg)
        });
        // Construct record
        Self {
            group,
            first,
            last,
            attend,
            meal,
            msg,
        }
    }
}
