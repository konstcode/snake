use std::{time::Duration, usize};

use rand::{thread_rng, Rng};

use crate::{frame::Drawable, timer::Timer, NUM_COLS, NUM_ROWS};

const MIN_APPEAR_TIME: usize = 5;
const MAX_APPEAR_TIME: usize = 10;

struct Apple {
    x: usize,
    y: usize,
    timer: Timer,
}

struct AppleDispencer {
    deployed: Vec<Apple>,
    max_count: usize,
}

impl Apple {
    fn new(x: usize, y: usize) -> Self {
        let mut rng = thread_rng();
        let rand_time = rng.gen_range(MIN_APPEAR_TIME..=MAX_APPEAR_TIME);
        Self {
            x,
            y,
            timer: Timer::new(Duration::from_secs(rand_time as u64)),
        }
    }

    fn timer_update_check(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        if self.timer.finished() {
            return true;
        }
        false
    }
}

impl Drawable for Apple {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = 'Q';
    }
}

impl AppleDispencer {
    pub fn new(max_count: usize) -> Self {
        let deployed = Vec::new();
        Self {
            max_count,
            deployed,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if self.deployed.len() < self.max_count {
            self.deploy();
        }
        self.deployed
            .retain_mut(|apple| apple.timer_update_check(delta));
    }

    fn deploy(&mut self) {
        let mut rng = thread_rng();
        let rand_x = rng.gen_range(0..NUM_COLS);
        let rand_y = rng.gen_range(0..NUM_ROWS);
        let apple = Apple::new(rand_x, rand_y);
        self.deployed.push(apple);
    }
}

impl Drawable for AppleDispencer {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.deployed.iter().for_each(|apple| apple.draw(frame));
    }
}
