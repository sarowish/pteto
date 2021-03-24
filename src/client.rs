use crate::commands::{Command, Output};
use std::net::TcpStream;
use std::{io::prelude::*, net::Shutdown};

pub struct Client {
    socket: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        Client {
            socket: TcpStream::connect("127.0.0.1:7878").expect("couldn't connect to port"),
        }
    }

    pub fn add(&mut self, subject: String, seconds: u32) {
        self.run_command(Command::Add(subject, seconds));
    }

    pub fn toggle(&mut self) {
        self.run_command(Command::Toggle);
    }

    pub fn stop(&mut self) {
        self.run_command(Command::Stop);
    }

    pub fn status(&mut self) -> String {
        if let Output::Status(status) = self.run_command(Command::Status) {
            status
        } else {
            String::new()
        }
    }

    pub fn stats(&mut self) -> Vec<String> {
        if let Output::Stats(stats) = self.run_command(Command::Stats) {
            stats
        } else {
            Vec::new()
        }
    }

    pub fn change_subject(&mut self, subject: String) {
        self.run_command(Command::ChangeSubject(subject));
    }

    pub fn kill(&mut self) {
        self.run_command(Command::Kill);
    }

    fn run_command(&mut self, command: Command) -> Output {
        let command = bincode::serialize(&command).unwrap();
        self.socket.write_all(&command).unwrap();
        self.socket.shutdown(Shutdown::Write).unwrap();
        let mut reply = Vec::new();
        self.socket.read_to_end(&mut reply).unwrap();
        bincode::deserialize(&reply).unwrap()
    }
}
