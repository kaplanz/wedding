use std::fs::File;
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::user::User;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub ident: usize,
    pub group: usize,
    #[serde(flatten)]
    pub guest: User,
}

#[derive(Debug, Default)]
pub struct Database(Vec<Record>);

impl Database {
    #[allow(unused)]
    pub fn new(data: Vec<Record>) -> Self {
        Self(data)
    }
}

impl Deref for Database {
    type Target = Vec<Record>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Database {
    type Item = Record;

    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<&Path> for Database {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);
        let data = reader
            .deserialize()
            .collect::<Result<_, _>>()
            .map_err(Error::Csv)?;
        Ok(Database(data))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Path(#[from] io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
}
