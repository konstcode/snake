use std::time::Instant;

use crate::frame::Drawable;

pub struct TopBar {
    score: usize,
    instant: Instant,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            score: 0,
            instant: Instant::now(),
        }
    }

    pub fn scores(&mut self) {
        self.score += 1;
    }
}

impl Drawable for TopBar {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        let time = self.instant.elapsed().as_secs();
        let bar = format!(" Score: {:04} Time: {:05}", self.score, time);
        for (i, s) in bar.chars().enumerate() {
            frame[i][0] = s;
        }
    }
}
