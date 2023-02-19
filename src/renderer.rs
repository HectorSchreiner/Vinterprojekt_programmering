use line_drawing::Bresenham;

use crate::{game::GameRenderer, shapes::*};

pub const WIDTH: usize = 1500;
pub const HEIGHT: usize = 900;
pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 200;

pub const MAP_WIDTH_COUNT: i32 = 16;
pub const MAP_HEIGHT_COUNT: i32 = 16;

pub fn to_color(color: (u8, u8, u8)) -> u32 {
    let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    (r << 16) | (g << 8) | b
}

pub struct WindowRenderer {
    pub buffer: Vec<u32>,
}

impl WindowRenderer {
    pub fn pixel<T>(&mut self, position: T, color: (u8, u8, u8))
    where T: Into<Position2D> {
        let position = position.into();
        let x = position.x as usize;
        let y = position.y as usize;

        if x < WIDTH && x > 0 && y < HEIGHT && y > 0 {
            self.buffer[x + y * WIDTH] = to_color(color);
        }
    }

    pub fn rect<T: Into<Square>>(&mut self, square: T, color: (u8, u8, u8)) {
        let square = square.into();
        let pos_y = square.position.y as i32;
        let pos_x = square.position.x as i32;

        for y in pos_y..(square.height as i32 + pos_y) {
            for x in pos_x..(square.length as i32 + pos_x) {
                self.buffer[(y * WIDTH as i32 + x) as usize] = to_color(color);
            }
        }
    }

    pub fn line(&mut self, line: &Line, color: (u8, u8, u8)) {
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.pixel((x as f32, y as f32), (color.0, color.1, color.2));
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = to_color(color);
        }
    }
}
