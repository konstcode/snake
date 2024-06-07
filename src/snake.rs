use std::time::Duration;

use crate::{
    apple::AppleDispencer,
    frame::{Drawable, Frame},
    timer::Timer,
    Point, NUM_COLS, NUM_ROWS,
};

const SPEED: u64 = 500;

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    timer: Timer,
    alive: bool,
    can_turn: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Snake {
    pub fn new() -> Self {
        let head = Point::new(20, 20);
        let body = vec![
            head,
            Point::new(head.x - 1, head.y),
            Point::new(head.x - 2, head.y),
            Point::new(head.x - 3, head.y),
        ];
        Self {
            body,
            direction: Direction::Right,
            timer: Timer::new(Duration::from_millis(SPEED)),
            alive: true,
            can_turn: true,
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.timer.reset();
            self.next_move();
            self.can_turn = true;
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
    fn move_or_dead(&mut self, next_section: &mut Point) {
        match self.direction {
            Direction::Left if next_section.x > 0 => next_section.x -= 1,
            Direction::Left if next_section.x == 0 => self.alive = false,
            Direction::Up if next_section.y > 0 => next_section.y -= 1,
            Direction::Up if next_section.y == 0 => self.alive = false,
            Direction::Right if next_section.x < NUM_COLS - 1 => next_section.x += 1,
            Direction::Right if next_section.x == NUM_COLS - 1 => self.alive = false,
            Direction::Down if next_section.y < NUM_ROWS - 1 => next_section.y += 1,
            Direction::Down if next_section.y == NUM_ROWS - 1 => self.alive = false,
            _ => (),
        };
    }
    pub fn is_dead(&self) -> bool {
        !self.alive
    }
    //Turn left or rigth relative to current direction, plus
    //can turn only once a move
    pub fn turn_if_possible(&mut self, new_dirrection: Direction) {
        if self.can_turn == false {
            return;
        }
        self.direction = match (self.direction, new_dirrection) {
            (Direction::Left, x) if x == Direction::Up || x == Direction::Down => new_dirrection,
            (Direction::Up, x) if x == Direction::Left || x == Direction::Right => new_dirrection,
            (Direction::Right, x) if x == Direction::Up || x == Direction::Down => new_dirrection,
            (Direction::Down, x) if x == Direction::Left || x == Direction::Right => new_dirrection,
            (_, _) => return,
        };
        self.can_turn = false;
    }
    pub fn check_if_ate_apple(&mut self, &mut dispencer: AppleDispencer) {
        for snake_part in self.body {
            snake_part.x
        }
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        let head_char = match self.direction {
            Direction::Left => '⇐',
            Direction::Up => '⇑',
            Direction::Right => '⇒',
            Direction::Down => '⇓',
        };
        frame[self.body[0].x][self.body[0].y] = head_char;
        let tail = self.body.split_first_chunk::<1>().unwrap().1;
        for s in tail {
            frame[s.x][s.y] = 'X';
        }
    }
}

