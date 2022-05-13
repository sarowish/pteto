use crate::commands::{Command, Output};
use crate::database::{Database, DatabaseFile};
use crate::entry::Entry;
use crate::subject::Subject;
use crate::timer::Timer;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Server {
    subject: String,
    subjects: HashMap<String, Subject>,
    timer: Arc<Mutex<Timer>>,
    database: Database,
}

impl Server {
    pub fn new(subject_list: &[&str]) -> Self {
        let mut subjects = HashMap::new();
        for &subject in subject_list.iter() {
            let subject = subject.to_string();
            subjects.insert(subject.clone(), Subject::new(subject));
        }

        let mut file = DatabaseFile::new(PathBuf::from("./times.pt"));
        let db = file.open().unwrap();

        for entry in db.entries.iter() {
            if entry.is_todays_entry() {
                subjects
                    .get_mut(&entry.subject())
                    .unwrap()
                    .add_seconds(entry.seconds());
            }
        }

        Server {
            subject: subject_list[0].to_string(),
            subjects,
            timer: Arc::new(Mutex::new(Timer::new())),
            database: db,
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7878").expect("couldn't bind to port");
        let tick = Arc::clone(&self.timer);
        thread::spawn(move || loop {
            tick.lock().unwrap().tick();
            thread::sleep(Duration::from_secs(1));
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
            Command::Add(subject, seconds) => {
                self.add(subject, seconds);
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
            Command::Status => Output::Status(self.status()),
            Command::Stats => Output::Stats(self.stats()),
            Command::ChangeSubject(subject) => {
                self.change_subject(subject);
                Output::ChangeSubject
            }
            Command::Kill => Output::Kill,
        }
    }

    fn toggle(&self) {
        self.timer.lock().unwrap().toggle();
    }

    fn stop(&mut self) {
        let seconds = self.timer.lock().unwrap().stop();
        self.add(self.subject.clone(), seconds);
    }

    fn add(&mut self, mut subject: String, seconds: u32) {
        if subject.is_empty() {
            subject = self.subject.clone();
        }
        self.subjects
            .get_mut(&subject)
            .unwrap()
            .add_seconds(seconds);
        self.database.entries.push(Entry::new(subject, seconds));
        self.database.modified = true;
    }

    fn change_subject(&mut self, subject: String) {
        self.subject = subject;
    }

    fn status(&self) -> String {
        format!(
            "{}: {} - {}",
            if self.timer.lock().unwrap().is_running() {
                "Running"
            } else {
                "Paused"
            },
            self.subject,
            self.timer.lock().unwrap()
        )
    }

    fn stats(&self) -> Vec<String> {
        let mut total = 0;
        let mut res: Vec<String> = self
            .subjects
            .values()
            .map(|v| {
                total += v.seconds;
                format!("{}", v)
            })
            .collect();
        res.push(format!("Total - {}", crate::utils::seconds_to_clock(total)));
        res
    }
}
