use std::{time::Duration, usize};

use rand::{thread_rng, Rng};

use crate::{frame::Drawable, timer::Timer, Point, NUM_COLS, NUM_ROWS};

const MIN_APPEAR_TIME: usize = 10;
const MAX_APPEAR_TIME: usize = 20;

#[derive(Clone, Copy)]
struct Apple {
    place: Point,
    timer: Timer,
}

pub struct AppleDispencer {
    deployed: Vec<Apple>,
    max_count: u8,
}

impl Apple {
    fn new(place: Point) -> Self {
        let mut rng = thread_rng();
        let rand_time = rng.gen_range(MIN_APPEAR_TIME..=MAX_APPEAR_TIME);
        Self {
            place,
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
    pub fn get_position(&self) -> Point {
        self.place
    }
}

impl Drawable for Apple {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.place.x][self.place.y] = 'Q';
    }
}

impl AppleDispencer {
    pub fn new(max_count: u8) -> Self {
        let deployed = Vec::new();
        Self {
            max_count,
            deployed,
        }
    }
    pub fn update(&mut self, delta: Duration) {
        if (self.deployed.len() as u8) < self.max_count {
            self.deploy();
        }
        self.deployed
            .retain_mut(|apple| !apple.timer_update_check(delta));
    }
    pub fn eat_apples_if<F>(&mut self, mut condition: F)
    where
        F: FnMut(&Point) -> bool,
    {
        self.deployed.retain(|p| !condition(&p.get_position()));
    }
    fn deploy(&mut self) {
        let mut rng = thread_rng();
        let rand_x = rng.gen_range(0..NUM_COLS);
        //Note: start from 1 to avoid topbar spam, temporary solution for now
        let rand_y = rng.gen_range(1..NUM_ROWS);
        let apple = Apple::new(Point::new(rand_x, rand_y));
        self.deployed.push(apple);
    }
}

impl Drawable for AppleDispencer {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.deployed.iter().for_each(|apple| apple.draw(frame));
    }
}
