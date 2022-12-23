use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use log::{debug, trace};
use thiserror::Error;
use uuid::Uuid;

use crate::user::User;

pub mod guest;
mod record;

use self::guest::{Guest, Reply};
use self::record::Record;

pub type Ident = Uuid;
pub type Group = usize;

#[derive(Clone, Debug, Default)]
pub struct Database {
    pub path: Option<PathBuf>,
    idents: HashMap<User, Ident>,
    guests: HashMap<Ident, Guest>,
    groups: HashMap<Group, Vec<Ident>>,
}

impl Database {
    pub fn new(guests: Vec<Guest>) -> Self {
        let mut db = Database::default();

        // Use the guests to populate the database
        for mut guest in guests {
            trace!(
                "read: `{}`, rsvp: {}",
                guest.user().name(),
                guest.rsvp.is_some()
            );
            // Create an identifier for this guest
            let ident = Ident::new_v4();
            guest.user.ident = ident;
            // Get the guest's group
            let group = guest.group;
            // Add the user's identifier to the database
            let user = guest.user.clone();
            db.idents.insert(user, ident);
            // Add the guest into the database
            db.guests.insert(ident, guest);
            // Insert the guest (by identifier) into their group
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

    pub fn iter(&self) -> impl Iterator<Item = &User> {
        self.guests.values().map(|guest| guest.user())
    }

    pub fn len(&self) -> usize {
        self.guests.len()
    }

    pub fn query(&self, user: &User) -> Option<Ident> {
        self.idents.get(user).cloned()
    }

    pub fn update(&mut self, user: &User, reply: Reply) -> Result<(), Error> {
        // Get the user's identifier
        let ident = user.ident;
        // Extract the guest to update
        let guest = self
            .guests
            .get_mut(&ident)
            .ok_or_else(|| Error::Guest(ident))?;
        // Convert the reply into an rsvp
        let rsvp = reply.into();
        // Perform the update
        trace!("update: `{}` -> {}", guest.user().name(), rsvp);
        guest.update(rsvp);

        Ok(())
    }

    pub fn write(&self) -> Result<(), Error> {
        // Open the output file
        let path = self.path.as_ref().ok_or(Error::Path)?;
        let mut writer = csv::Writer::from_path(path)?;
        debug!("writing: `{}`", path.display());
        // Write the database
        for guest in self.guests.values() {
            // Convert the guest into a writable record
            let record = Record::from(guest.clone());
            // Serialize and write it
            writer.serialize(record).map_err(Error::Csv)?;
            trace!(
                "wrote: `{}`, rsvp: {}",
                guest.user().name(),
                guest.rsvp.is_some()
            );
        }

        Ok(())
    }
}

impl TryFrom<&Path> for Database {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        // Open the input file
        let mut reader = csv::Reader::from_path(path)?;
        // Read the guests
        debug!("reading: `{}`", path.display());
        let data: Vec<Guest> = reader
            .deserialize::<Record>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::Csv)?
            .into_iter()
            .map(Into::into)
            .collect();
        // Construct a database
        Ok(Database::new(data))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing path")]
    Path,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("missing guest: {0}")]
    Guest(Ident),
}
