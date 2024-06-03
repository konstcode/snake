use std::time::Duration;

/// Simple Timer to check if it finished using delta form
/// main thread instance.
pub struct Timer {
    total: Duration,
    max: Duration,
    finished: bool,
}

impl Timer {
    pub fn new(time: Duration) -> Self {
        Self {
            total: Duration::new(0, 0),
            max: time,
            finished: false,
        }
    }

    pub fn tick(&mut self, time: Duration) {
        self.total += time;
        match self.total >= self.max {
            true => self.finished = true,
            _ => (),
        };
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.total = Duration::new(0, 0);
        self.finished = false;
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::timer::Timer;

    #[test]
    fn test_timer() {
        let mut timer = Timer::new(Duration::new(1, 0));
        timer.tick(Duration::from_millis(500));
        assert!(!timer.finished());
        timer.tick(Duration::from_millis(500));
        assert!(timer.finished());
        println!("{:?}", timer.total);
        timer.reset();
        assert!(!timer.finished());
    }
}
