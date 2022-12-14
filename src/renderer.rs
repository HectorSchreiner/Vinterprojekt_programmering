use line_drawing::Bresenham;

use crate::shapes::*;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

pub fn to_color(color: (u8, u8, u8)) -> u32 {
    let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    (r << 16) | (g << 8) | b
}

pub struct WindowRenderer {
    pub buffer: Vec<u32>,
}

impl WindowRenderer {
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
        let pos_y: i32 = square.position.y;
        let pos_x: i32 = square.position.x;

        for y in pos_y..square.height as i32 + pos_y {
            for x in pos_x..square.length as i32 + pos_x {
                self.buffer[(y * WIDTH as i32 + x) as usize] = to_color(color);
            }
        }
    }

    pub fn line(&mut self, line: &Line, color: (u8, u8, u8)) {
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.pixel((x, y), (color.0, color.1, color.2));
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = to_color(color);
        }
    }
}
