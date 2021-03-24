use std::fmt;

pub struct Subject {
    pub subject: String,
    pub seconds: u32,
}

impl Subject {
    pub fn new(subject: String) -> Self {
        Subject {
            subject,
            seconds: 0,
        }
    }

    pub fn add_seconds(&mut self, seconds: u32) {
        self.seconds += seconds;
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.subject,
            crate::utils::seconds_to_clock(self.seconds)
        )
    }
}
