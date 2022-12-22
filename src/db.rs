use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use log::{debug, trace};
use serde::Deserialize;
use thiserror::Error;

use crate::guest::{Guest, Rsvp};
use crate::user::User;

type Ident = usize;

#[derive(Debug, Deserialize)]
pub struct Invitee {
    group: usize,
    #[serde(flatten)]
    user: User,
}

#[derive(Clone, Debug, Default)]
pub struct Database {
    guests: HashMap<Ident, Guest>,
    groups: HashMap<Ident, Vec<Ident>>,
}

impl Database {
    pub fn new(invitees: Vec<Invitee>) -> Self {
        let mut db = Database::default();

        // Use the invitees to populate the database
        for (ident, invitee) in invitees.into_iter().enumerate() {
            trace!("found guest: `{}`", invitee.user.name());
            // Convert each invitee into a guest
            let guest = invitee.user.into();
            db.guests.insert(ident, guest);
            // Add the guest (by identifier) into their group
            let group = invitee.group;
            db.groups.entry(group).or_default().push(ident);
        }

        // Return the new database
        debug!(
            "database: {} guests, {} groups",
            db.guests.len(),
            db.groups.len()
        );
        db
    }

    pub fn len(&self) -> usize {
        self.guests.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &User> {
        self.guests.values().map(|guest| &guest.user)
    }

    pub fn update(&mut self, ident: Ident, rsvp: Rsvp) -> Result<(), Error> {
        // Extract the guest to update
        let guest = self
            .guests
            .get_mut(&ident)
            .ok_or_else(|| Error::Missing(ident))?;
        trace!("update: `{}` -> {}", guest.user().name(), rsvp);
        // Perform the update
        guest.rsvp(rsvp);

        Ok(())
    }
}

impl TryFrom<&Path> for Database {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        debug!("reading guestlist: `{}`", path.display());
        // Read and parse the invitees
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);
        let data: Vec<Invitee> = reader
            .deserialize()
            .collect::<Result<_, _>>()
            .map_err(Error::Csv)?;
        // Construct a database
        Ok(Database::new(data))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Path(#[from] io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("missing guest: {0}")]
    Missing(Ident),
}
