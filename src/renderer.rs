use crate::shapes::*;
use line_drawing::Bresenham;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

pub fn to_color(color: (u8, u8, u8)) -> u32 {
    let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    (r << 16) | (g << 8) | b
}

pub struct Renderer {
    pub buffer: Vec<u32>,
}

impl Renderer {
    pub fn pixel<T: Into<Position2D>>(&mut self, position: T, color: (u8, u8, u8)) {
        let position = position.into();
        let x = position.x as usize;
        let y = position.y as usize;

        if x < WIDTH && x > 0 && y < HEIGHT && y > 0 {
            self.buffer[x + y * WIDTH] = to_color(color);
        }
    }

    pub fn rect<T: Into<Square>>(&mut self, square: T, color: (u8, u8, u8)) {
        let square = square.into();
        let pos_y = square.position.y;
        let pos_x = square.position.x;

        for y in pos_y..square.height + pos_y {
            for x in pos_x..square.lenght + pos_x {
                self.buffer[(y * WIDTH as u32 + x) as usize] = to_color(color);
            }
        }
    }

    pub fn line(&mut self, line: &Line, color: (u8, u8, u8)) {
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.pixel((x as u32, y as u32), (color.0, color.1, color.2));
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = to_color(color);
        }
    }
}
