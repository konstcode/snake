pub mod apple;
pub mod audio;
pub mod frame;
pub mod render;
pub mod snake;
pub mod timer;

pub const NUM_ROWS: usize = 30;
pub const NUM_COLS: usize = 50;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

