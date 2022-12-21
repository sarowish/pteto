use std::fmt;

pub struct Timer {
    running: bool,
    seconds: u32,
    pub exceeded: bool,
}

impl Timer {
    pub fn new(duration: u32) -> Self {
        Timer {
            running: false,
            seconds: duration,
            exceeded: false,
        }
    }

    pub fn reset(&mut self, duration: u32) {
        self.seconds = duration;
        self.running = false;
        self.exceeded = false;
    }

    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    pub fn tick(&mut self) {
        if self.running {
            if self.exceeded {
                self.seconds += 1;
            } else {
                self.seconds -= 1;
            }
        }
    }

    pub fn stop(&mut self) -> u32 {
        self.running = false;
        std::mem::take(&mut self.seconds)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::utils::length_as_hhmmss(self.seconds()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_basic() {
        let mut timer = Timer::new(30);
        timer.toggle();
        assert!(timer.is_running());
        timer.tick();
        timer.tick();
        assert_eq!(timer.seconds(), 28);
        let res = timer.stop();
        assert!(!timer.is_running());
        assert_eq!(timer.seconds(), 0);
        assert_eq!(res, 28);
    }
}
