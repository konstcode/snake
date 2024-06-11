pub mod apple;
pub mod audio;
pub mod frame;
pub mod menu;
pub mod render;
pub mod snake;
pub mod timer;
pub mod topbar;

pub const NUM_ROWS: usize = 30;
pub const NUM_COLS: usize = 30;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
