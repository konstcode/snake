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
    last_score: usize,
    last_time: usize,
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
            last_time: 0,
            last_score: 0,
        }
    }

    pub fn insrease_current_option(&mut self) {
        match self.current_option {
            MenuOption::Speed => self.increase_speed(),
            MenuOption::Apples => self.increase_apples(),
        }
    }
    pub fn decrease_current_option(&mut self) {
        match self.current_option {
            MenuOption::Speed => self.decrease_speed(),
            MenuOption::Apples => self.decrease_apples(),
        }
    }
    pub fn switch_current_option(&mut self) {
        self.current_option = match self.current_option {
            MenuOption::Speed => MenuOption::Apples,
            MenuOption::Apples => MenuOption::Speed,
        }
    }
    pub fn apples(&self) -> u8 {
        self.max_apples
    }
    pub fn speed(&self) -> u64 {
        self.speed_millis
    }
    pub fn get_game_results(&mut self, scores: usize, time: usize) {
        self.last_score = scores;
        self.last_time = time;
    }
    fn increase_speed(&mut self) {
        let new_speed = self.speed_millis + 50;
        if new_speed <= SPEED_LIMITS.1 {
            self.speed_millis = new_speed;
        }
    }

    fn increase_apples(&mut self) {
        let new_max_apples = self.max_apples + 1;
        if new_max_apples <= APPLE_LIMITS.1 {
            self.max_apples = new_max_apples;
        }
    }

    fn decrease_speed(&mut self) {
        let new_speed = self.speed_millis - 50;
        if new_speed >= SPEED_LIMITS.0 {
            self.speed_millis = new_speed;
        }
    }

    fn decrease_apples(&mut self) {
        let new_max_apples = self.max_apples - 1;
        if new_max_apples >= APPLE_LIMITS.0 {
            self.max_apples = new_max_apples;
        }
    }
}
// Depending on current option we put selection and
// try format it some normal view
impl Drawable for Menu {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        // Last game scores and time
        if self.last_score > 0 {
            draw_text(
                frame,
                Point::new(1, 1),
                format!("Last score: {} Time: {}", self.last_score, self.last_time).as_str(),
            );
        }

        // Speed and Apples options
        draw_text(frame, Point::new(10, 10), "Refresh rate:");
        let mut speed_option = format!("  {:^6}  ", self.speed_millis);
        let mut apple_option = format!("  {:^6}  ", self.max_apples);
        match self.current_option {
            MenuOption::Speed => speed_option = format!("< {:^6} >", self.speed_millis),
            MenuOption::Apples => apple_option = format!("< {:^6} >", self.max_apples),
        }

        draw_text(frame, Point::new(10, 11), speed_option.as_str());
        draw_text(
            frame,
            Point::new(10, 12),
            format!("{:^6}", "Apples:").as_str(),
        );
        draw_text(frame, Point::new(10, 13), apple_option.as_str());
        draw_text(
            frame,
            Point::new(6, 14),
            format!("{:^6}", "Press Enter to Start").as_str(),
        );
    }
}
