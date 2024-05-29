use crate::frame::{Drawable, Frame};

pub struct Snake {
    head: Section,
    tail: Vec<Section>,
    direction: Direction,
}

enum Direction {
    Left,
    Top,
    Right,
    Down
}
struct Section (usize, usize);

impl Snake {
    pub fn new()  -> Self {
        let head = Section(20, 20);
        let tail = vec![Section(head.0 - 1, head.1),
                                     Section(head.0 - 2, head.1),
                                     Section(head.0 - 3, head.1)];
        Self {
            head: head,
            tail: tail,
            direction: Direction::Right
        }
    }

    pub fn turn_left() {

    }
}
impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        let head_char = match self.direction {
            Direction::Left => '⇐',
            Direction::Top => '⇑',
            Direction::Right => '⇒',
            Direction::Down => '⇓',
        };
        frame[self.head.0][self.head.1] = head_char;
        for s in &self.tail {
            frame[s.0][s.1] = 'X';
        }
    }
}