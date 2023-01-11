use crate::{renderer, shapes::*};
use crate::{Renderer, HEIGHT, WIDTH};

pub struct Game<'a> {
    renderer: &'a mut Renderer,
}

impl Game<'_> {
    pub fn render_map(&mut self, mapRow: u32, mapCol: u32, gridSize: u32) {
        let spacing = 2;

        for y in 0..mapCol {
            for x in 0..mapRow {
                self.renderer.rect(
                    (gridSize, gridSize, (x * gridSize, y * gridSize)),
                    (0, 0, 0),
                );

                self.renderer.rect(
                    (
                        gridSize - spacing,
                        gridSize - spacing,
                        (spacing / 2 + x * gridSize, spacing / 2 + y * gridSize),
                    ),
                    (255, 255, 255),
                );
            }
        }
    }
}
