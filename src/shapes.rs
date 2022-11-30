pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl From<(u32, u32)> for Position {
    fn from(position: (u32, u32)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}

pub struct Square {
    pub lenght: u32,
    pub height: u32,
    pub position: Position,
}
impl From<(u32, u32, Position)> for Square {
    fn from(square: (u32, u32, Position)) -> Self {
        Self {
            lenght: square.0,
            height: square.1,
            position: square.2,
        }
    }
}

pub struct Line {
    pub pos_1: Position,
    pub pos_2: Position,
}
impl Line {
    pub fn new(pos_1: impl Into<Position>, pos_2: impl Into<Position>) -> Line {
        Line {
            pos_1: pos_1.into(),
            pos_2: pos_2.into(),
        }
    }

    pub fn length_of_line(&self) -> f32 {
        let dx: f32 = (self.pos_1.x as f32 - self.pos_2.x as f32).abs();
        let dy: f32 = (self.pos_1.y as f32 - self.pos_2.y as f32).abs();
        return (dx * dx + dy * dy).powf(0.5);
    }
}
