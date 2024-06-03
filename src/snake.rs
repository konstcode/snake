use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    timer::{self, Timer},
};

const SPEED: u64 = 300;

pub struct Snake {
    body: Vec<Section>,
    direction: Direction,
    timer: Timer,
}

enum Direction {
    Left,
    Top,
    Right,
    Down,
}

#[derive(Clone, Copy)]
struct Section(usize, usize);

impl Snake {
    pub fn new() -> Self {
        let head = Section(20, 20);
        let body = vec![
            head,
            Section(head.0 - 1, head.1),
            Section(head.0 - 2, head.1),
            Section(head.0 - 3, head.1),
        ];
        Self {
            body,
            direction: Direction::Right,
            timer: Timer::new(Duration::from_millis(SPEED)),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.timer.reset();
            self.next_move();
        }
    }

    fn next_move(&mut self) {
        let mut new_head_position = self.body[0];
        match self.direction {
            Direction::Left => new_head_position.0 -= 1,
            Direction::Top => new_head_position.1 -= 1,
            Direction::Right => new_head_position.0 += 1,
            Direction::Down => new_head_position.1 += 1,
        }
        self.body.insert(0, new_head_position);
        self.body.pop();
    }

    pub fn turn_left() {}
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        let head_char = match self.direction {
            Direction::Left => '⇐',
            Direction::Top => '⇑',
            Direction::Right => '⇒',
            Direction::Down => '⇓',
        };
        frame[self.body[0].0][self.body[0].1] = head_char;
        let tail = self.body.split_first_chunk::<1>().unwrap().1;
        for s in tail {
            frame[s.0][s.1] = 'X';
        }
    }
}

