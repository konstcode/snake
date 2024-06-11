use crate::Point;
use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = [[char; NUM_ROWS]; NUM_COLS];

pub fn new_frame() -> Frame {
    [[' '; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub trait Reset {
    fn reset(&mut self);
}

pub fn draw_text(frame: &mut Frame, start_at: Point, text: &str) {
    for (index, char) in text.chars().enumerate() {
        frame[start_at.x + index][start_at.y] = char;
    }
}
