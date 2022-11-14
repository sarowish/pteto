use std::sync::{Arc, Mutex};

use crate::timer::Timer;

pub enum Mode {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct Session {
    label: Option<String>,
    pub mode: Mode,
    pub timer: Arc<Mutex<Timer>>,
    pub work_length: u32,
    short_break_length: u32,
    long_break_length: u32,
}

impl Session {
    pub fn new() -> Self {
        let work_length = 25 * 60;

        Self {
            label: None,
            mode: Mode::Work,
            timer: Arc::new(Mutex::new(Timer::new(work_length))),
            work_length,
            short_break_length: 5 * 60,
            long_break_length: 15 * 60,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;

        let mut timer = self.timer.lock().unwrap();
        timer.reset(match self.mode {
            Mode::Work => self.work_length,
            Mode::ShortBreak => self.short_break_length,
            Mode::LongBreak => self.long_break_length,
        });
    }

    pub fn on_break(&self) -> bool {
        !matches!(self.mode, Mode::Work)
    }

    pub fn time_passed(&self) -> u32 {
        let mut timer = self.timer.lock().unwrap();

        if timer.exceeded {
            self.work_length + timer.stop()
        } else {
            self.work_length - timer.stop()
        }
    }

    pub fn get_label(&self) -> String {
        if let Some(label) = &self.label {
            label.clone()
        } else {
            "Unlabelled".to_string()
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = if label.is_empty() {
            None
        } else {
            Some(label.to_string())
        }
    }
}
