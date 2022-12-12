pub struct Color {
    pub rgb: (u8, u8, u8)
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self { rgb: rgb.into() }
    }
}

pub struct Position2D {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Position2D {
    fn from(position: (usize, usize)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}

pub struct Square {
    pub length: usize,
    pub height: usize,
    pub position: Position2D,
}

impl Square {
    pub fn new(length: usize, height: usize, position: Position2D) -> Self {
        Self {
            length,
            height,
            position,
        }
    }
}

impl<P: Into<Position2D>> From<(usize, usize, P)> for Square {
    fn from(square: (usize, usize, P)) -> Self {
        Self {
            length: square.0,
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
