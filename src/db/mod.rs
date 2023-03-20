use std::fmt::Display;
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use log::{debug, info, trace};
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

use crate::user::User;

pub mod guest;

use self::guest::{Guest, Reply};

pub type Group = usize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Ident(Uuid);

impl Default for Ident {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Deref for Ident {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Default)]
pub struct Database {
    pub path: Option<PathBuf>,
    idents: IndexMap<User, Ident>,
    guests: IndexMap<Ident, Guest>,
    groups: IndexMap<Group, Vec<Ident>>,
}

impl Database {
    pub fn new(guests: Vec<Guest>) -> Self {
        let mut db = Database::default();

        // Use the guests to populate the database
        for guest in guests {
            trace!(
                "read: `{}`, reply: {}",
                guest.user().name(),
                guest
                    .reply()
                    .as_ref()
                    .map_or_else(|| "none".to_string(), |reply| format!("{reply}"))
            );
            // Get the identifier for this guest
            let ident = guest.user().ident;
            // Get the guest's group
            let group = guest.group();
            // Add the user's identifier to the database
            let user = guest.user().clone();
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
        self.guests.values().map(Guest::user)
    }

    pub fn len(&self) -> usize {
        self.guests.len()
    }

    pub fn query(&self, user: &User) -> Option<&Ident> {
        self.idents.get(user)
    }

    pub fn guest(&self, ident: &Ident) -> Option<&Guest> {
        self.guests.get(ident)
    }

    pub fn group(&self, ident: &Ident) -> Result<&[Ident], Error> {
        // Extract the guest
        let group = self.guests.get(ident).ok_or(Error::Guest)?.group();
        // Return the user's group
        self.groups
            .get(&group)
            .map(Vec::as_slice)
            .ok_or(Error::Guest)
    }

    pub fn update(&mut self, ident: &Ident, reply: Reply) -> Result<(), Error> {
        // Extract the guest to update
        let guest = self.guests.get_mut(ident).ok_or(Error::Guest)?;
        // Perform the update
        info!("update: `{}` -> {reply}", guest.user(),);
        guest.update(reply);

        Ok(())
    }

    pub fn write(&self) -> Result<(), Error> {
        // Open the output file
        let path = self.path.as_ref().ok_or(Error::Path)?;
        let mut writer = csv::Writer::from_path(path)?;
        debug!("writing: `{}`", path.display());
        // Write the database
        for guest in self.guests.values() {
            // Serialize and write each guest
            writer.serialize(guest).map_err(Error::Csv)?;
            trace!(
                "wrote: `{}`, reply: {}",
                guest.user(),
                guest
                    .reply()
                    .as_ref()
                    .map_or_else(|| "none".to_string(), |reply| format!("{reply}"))
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
            .deserialize::<Guest>()
            .collect::<Result<_, _>>()
            .map_err(Error::Csv)?;
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
    #[error("missing guest")]
    Guest,
}
