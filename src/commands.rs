use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Command {
    Add(String, u32),
    Toggle,
    Stop,
    Status,
    Stats,
    ChangeSubject(String),
    Kill,
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    Add,
    Toggle,
    Stop,
    Status(String),
    Stats(Vec<String>),
    ChangeSubject,
    Kill,
}
