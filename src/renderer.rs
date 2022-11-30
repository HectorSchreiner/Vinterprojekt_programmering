use crate::shapes::*;
use crate::utils::*;
use line_drawing::Bresenham;

pub struct Renderer {
    pub buffer: Vec<u32>,
}

impl Renderer {
    pub fn pixel(&mut self, position: (usize, usize), color: (u8, u8, u8)) {
        if position.0 < WIDTH && position.0 > 0 && position.1 < HEIGHT && position.1 > 0 {
            self.buffer[position.0 + position.1 * WIDTH] =
                tuple_to_rgb(color);
        }
    }

    pub fn rect(&mut self, square: &Square, color: (u8, u8, u8)) {
        let pos_y = square.position.y;
        let pos_x = square.position.x;

        for y in pos_y..square.height + pos_y {
            for x in pos_x..square.lenght + pos_x {
                self.buffer[(y * WIDTH as u32 + x) as usize] =
                    tuple_to_rgb(color);
            }
        }
    }

    pub fn line(&mut self, line: &Line, color: (u8, u8, u8)) {
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.pixel(
                (x as usize, y as usize),
                (color.0, color.1, color.2),
            );
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = tuple_to_rgb(color);
        }
    }
}
