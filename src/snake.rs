use std::time::Duration;

use crate::{
    apple::AppleDispencer,
    frame::{Drawable, Frame},
    timer::Timer,
    Point, NUM_COLS, NUM_ROWS,
};

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    timer: Timer,
    alive: bool,
    can_turn: bool,
    adding_tail: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Snake {
    pub fn new(speed: u64) -> Self {
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
            timer: Timer::new(Duration::from_millis(speed)),
            alive: true,
            can_turn: true,
            adding_tail: false,
        }
    }
    pub fn update<F>(&mut self, delta: Duration, mut do_if_move: F)
    where
        F: FnMut(),
    {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.timer.reset();
            self.next_move();
            do_if_move();
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
        if self.adding_tail == true {
            self.adding_tail = false;
        } else {
            self.body.pop();
        }
    }
    fn move_or_dead(&mut self, next_section: &mut Point) {
        if self.is_boarder_next(next_section) {
            self.alive = false;
            return;
        }
        match self.direction {
            Direction::Left => next_section.x -= 1,
            Direction::Up => next_section.y -= 1,
            Direction::Right => next_section.x += 1,
            Direction::Down => next_section.y += 1,
        };
        if self.is_tail_next(next_section) {
            self.alive = false;
            return;
        }
    }
    fn is_boarder_next(&self, next_section: &mut Point) -> bool {
        match (self.direction, next_section) {
            (Direction::Left, Point { x: 0, .. }) => true,
            //Note: start from 1 to avoid topbar, temporary solution for now
            (Direction::Up, Point { y: 1, .. }) => true,
            (Direction::Right, Point { x, .. }) if *x == NUM_COLS - 1 => true,
            (Direction::Down, Point { y, .. }) if *y == NUM_ROWS - 1 => true,
            _ => false,
        }
    }
    fn is_tail_next(&self, next_section: &mut Point) -> bool {
        for p in &self.body {
            if *p == *next_section {
                return true;
            }
        }
        false
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
    pub fn check_if_ate_apple<W>(&mut self, dispencer: &mut AppleDispencer, mut do_if_ate: W)
    where
        W: FnMut(),
    {
        for snake_part in &self.body {
            dispencer.eat_apples_if(|p| {
                if p == snake_part {
                    self.adding_tail = true;
                    do_if_ate();
                    return true;
                }
                return false;
            });
        }
    }
    pub fn is_growing(&self) -> bool {
        self.adding_tail
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
