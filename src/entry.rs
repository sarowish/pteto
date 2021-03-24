use anyhow::{Context, Result};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    subject: String,
    seconds: u32,
    date: NaiveDate,
}

impl Entry {
    pub fn new(subject: String, seconds: u32) -> Self {
        Self {
            subject,
            seconds,
            date: Utc::today().naive_utc(),
        }
    }

    pub fn subject(&self) -> String {
        self.subject.clone()
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }

    pub fn _date(&self) -> NaiveDate {
        self.date
    }

    pub fn is_todays_entry(&self) -> bool {
        self.date == Utc::today().naive_utc()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Entries(Vec<Entry>);

impl Entries {
    pub fn new() -> Self {
        Entries(Vec::new())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Entries> {
        bincode::deserialize(bytes).context("Couldn't deserialize bytes")
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).context("Couldn't serialize to bytes")
    }
}

impl Deref for Entries {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
