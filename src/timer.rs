use std::fmt;

pub struct Timer {
    state: bool,
    seconds: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            state: false,
            seconds: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
    }

    pub fn tick(&mut self) {
        if self.state {
            self.seconds += 1;
        }
    }

    pub fn stop(&mut self) -> u32 {
        self.state = false;
        std::mem::take(&mut self.seconds)
    }

    pub fn is_running(&self) -> bool {
        self.state
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::utils::seconds_to_clock(self.seconds()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_basic() {
        let mut timer = Timer::new();
        timer.toggle();
        assert!(timer.is_running());
        timer.tick();
        timer.tick();
        assert_eq!(timer.seconds(), 2);
        let res = timer.stop();
        assert!(!timer.is_running());
        assert_eq!(timer.seconds(), 0);
        assert_eq!(res, 2);
    }
}
