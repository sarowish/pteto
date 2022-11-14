use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Command {
    Add(String, u32),
    Toggle,
    Stop,
    Break(bool),
    Status,
    Stats,
    ChangeLabel(String),
    Kill,
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    Add,
    Toggle,
    Stop,
    Break,
    Status(String),
    Stats(Vec<String>),
    ChangeLabel,
    Kill,
}
