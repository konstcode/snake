use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    timer::{self, Timer},
    NUM_COLS, NUM_ROWS,
};

const SPEED: u64 = 300;

pub struct Snake {
    body: Vec<Section>,
    direction: Direction,
    timer: Timer,
    alive: bool,
}

pub enum Direction {
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
            alive: true,
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
        self.move_or_dead(&mut new_head_position);
        if self.is_dead() {
            return;
        }

        self.body.insert(0, new_head_position);
        self.body.pop();
    }

    fn move_or_dead(&mut self, next_section: &mut Section) {
        match self.direction {
            Direction::Left if next_section.0 > 0 => next_section.0 -= 1,
            Direction::Left if next_section.0 == 0 => self.alive = false,
            Direction::Top if next_section.1 > 0 => next_section.1 -= 1,
            Direction::Top if next_section.1 == 0 => self.alive = false,
            Direction::Right if next_section.0 < NUM_COLS - 1 => next_section.0 += 1,
            Direction::Right if next_section.0 == NUM_COLS - 1 => self.alive = false,
            Direction::Down if next_section.1 < NUM_ROWS - 1 => next_section.1 += 1,
            Direction::Down if next_section.1 == NUM_ROWS - 1 => self.alive = false,
            _ => (),
        };
    }

    pub fn is_dead(&self) -> bool {
        !self.alive
    }

    pub fn turn_if_possible(&self, dirrection: Direction) {}
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

