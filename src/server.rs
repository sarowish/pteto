use crate::commands::{Command, Output};
use crate::database::{Database, DatabaseFile};
use crate::entry::Entry;
use crate::session::{Mode, Session};
use notify_rust::Notification;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub struct Server {
    session: Session,
    times: HashMap<String, u32>,
    database: Database,
}

impl Server {
    pub fn new() -> Self {
        let mut file = DatabaseFile::new(PathBuf::from("./times.pt"));
        let db = file.open().unwrap();

        let mut times = HashMap::new();

        db.entries
            .iter()
            .filter(|entry| entry.is_todays_entry())
            .for_each(|entry| {
                let time = times.entry(entry.label()).or_default();
                *time += entry.seconds();
            });

        Server {
            session: Session::new(),
            times,
            database: db,
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7878").expect("couldn't bind to port");
        let timer = self.session.timer.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            {
                let mut timer = timer.lock().unwrap();
                timer.tick();
                if timer.seconds() == 0 {
                    timer.exceeded = true;
                    Notification::new().body("pteto: Time is up").show().unwrap();
                }
            }
        });
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            let command: Command = bincode::deserialize(&buffer).unwrap();
            let output = self.handle_command(command);
            stream
                .write_all(&bincode::serialize(&output).unwrap())
                .unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            if let Output::Kill = output {
                break;
            }
        }
    }

    fn handle_command(&mut self, command: Command) -> Output {
        match command {
            Command::Add(label, seconds) => {
                self.add(label, seconds);
                Output::Add
            }
            Command::Toggle => {
                self.toggle();
                Output::Toggle
            }
            Command::Stop => {
                self.stop();
                Output::Stop
            }
            Command::Break(long) => {
                self.take_break(long);
                Output::Break
            }
            Command::Status => Output::Status(self.status()),
            Command::Stats => Output::Stats(self.stats()),
            Command::ChangeLabel(label) => {
                self.change_label(label);
                Output::ChangeLabel
            }
            Command::Kill => Output::Kill,
        }
    }

    fn toggle(&self) {
        self.session.timer.lock().unwrap().toggle();
    }

    fn stop(&mut self) {
        if !self.session.on_break() {
            self.add_passed_time();
        }

        self.session.set_mode(Mode::Work);
    }

    fn take_break(&mut self, long: bool) {
        if self.session.on_break() {
            return;
        }

        self.add_passed_time();

        if long {
            self.session.set_mode(Mode::LongBreak);
        } else {
            self.session.set_mode(Mode::ShortBreak);
        }

        self.session.timer.lock().unwrap().toggle();
    }

    fn add_passed_time(&mut self) {
        let seconds = self.session.time_passed();
        self.add(self.session.get_label(), seconds);
    }

    fn add(&mut self, mut label: String, seconds: u32) {
        if seconds == 0 {
            return;
        }

        if label.is_empty() {
            label = self.session.get_label();
        }

        let time = self.times.entry(label.to_owned()).or_default();
        *time += seconds;

        self.database.entries.push(Entry::new(label, seconds));
        self.database.modified = true;
    }

    fn change_label(&mut self, label: String) {
        self.session.set_label(&label);
    }

    fn status(&self) -> String {
        let mut time = self.session.timer.lock().unwrap().to_string();

        if self.session.timer.lock().unwrap().exceeded {
            time.insert(0, '(');
            time.push(')');
        }

        format!(
            "{}: {} - {}",
            if self.session.timer.lock().unwrap().is_running() {
                "Running"
            } else {
                "Paused"
            },
            if matches!(self.session.mode, Mode::Work) {
                self.session.get_label()
            } else {
                "Break".to_string()
            },
            time
        )
    }

    fn stats(&self) -> Vec<String> {
        let mut total = 0;
        let mut res: Vec<String> = self
            .times
            .iter()
            .map(|(label, seconds)| {
                total += seconds;
                format!("{} - {}", label, crate::utils::length_as_hhmmss(*seconds))
            })
            .collect();
        res.push(format!("Total - {}", crate::utils::length_as_hhmmss(total)));
        res
    }
}
