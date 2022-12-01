pub struct Position2D {
    pub x: u32,
    pub y: u32,
}
impl From<(u32, u32)> for Position2D {
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
    pub position: Position2D,
}
impl<P: Into<Position2D>> From<(u32, u32, P)> for Square {
    fn from(square: (u32, u32, P)) -> Self {
        Self {
            lenght: square.0,
            height: square.1,
            position: square.2.into(),
        }
    }
}

pub struct Line {
    pub pos_1: Position2D,
    pub pos_2: Position2D,
}
impl Line {
    pub fn new(pos_1: impl Into<Position2D>, pos_2: impl Into<Position2D>) -> Line {
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
