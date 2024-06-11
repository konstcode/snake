use crate::{
    frame::{draw_text, Drawable},
    Point,
};

const SPEED_LIMITS: (u64, u64) = (100, 700);
const APPLE_LIMITS: (u8, u8) = (1, 10);

pub struct Menu {
    speed_millis: u64,
    max_apples: u8,
    pub active: bool,
    current_option: MenuOption,
}

pub enum MenuOption {
    Speed,
    Apples,
}

impl Menu {
    pub fn new(speed_millis: u64, max_apples: u8) -> Self {
        Self {
            speed_millis,
            max_apples,
            active: true,
            current_option: MenuOption::Speed,
        }
    }

    fn insrease_speed(&mut self) {
        let new_speed = self.speed_millis + 50;
        if new_speed < SPEED_LIMITS.1 {
            self.speed_millis += new_speed;
        }
    }
}

impl Drawable for Menu {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        // Speed option
        draw_text(frame, Point::new(10, 10), "Speed:");
        draw_text(frame, Point::new(10, 11), "Speed:");
    }
}
