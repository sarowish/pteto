use crate::entry::Entries;
use anyhow::Result;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Database {
    pub entries: Entries,
    pub modified: bool,
    path: PathBuf,
}

impl Database {
    pub fn save(&mut self) {
        if !self.modified {
            return;
        }
        let mut file = match fs::OpenOptions::new().write(true).open(self.path.clone()) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => fs::File::create(&self.path).unwrap(),
            Err(e) => panic!("{:?}", e),
        };
        file.write_all(&self.entries.to_bytes().unwrap()).unwrap();
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.save();
    }
}

pub struct DatabaseFile {
    file_path: PathBuf,
    buffer: Vec<u8>,
}

impl DatabaseFile {
    pub fn new(file_path: PathBuf) -> Self {
        DatabaseFile {
            file_path,
            buffer: Vec::new(),
        }
    }

    pub fn open(&mut self) -> Result<Database> {
        let entries = match fs::read(&self.file_path) {
            Ok(buffer) => {
                self.buffer = buffer;
                Entries::from_bytes(&self.buffer).unwrap()
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Entries::new(),
            Err(e) => panic!("{:?}", e),
        };
        Ok(Database {
            entries,
            modified: false,
            path: self.file_path.clone(),
        })
    }
}
