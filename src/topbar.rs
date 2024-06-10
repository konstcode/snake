use crate::frame::Drawable;

pub struct TopBar {
    score: usize,
}

impl TopBar {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn scores(&mut self) {
        self.score += 1;
    }
}

impl Drawable for TopBar {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        let bar = format!(" Score: {:04} ", self.score);
        for (i, s) in bar.chars().enumerate() {
            frame[i][0] = s;
        }
    }
}
