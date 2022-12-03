use crate::{renderer, shapes::*};
use crate::{Renderer, HEIGHT, WIDTH};

pub struct Game {}

impl Game {
    pub fn render_map(renderer: &mut Renderer, mapRow: u32, mapCol: u32, gridSize: u32) {
        let spacing = 2;

        for y in 0..mapCol {
            for x in 0..mapRow {
                renderer.rect(
                    (gridSize, gridSize, (x * gridSize, y * gridSize)),
                    (0, 0, 0),
                );

                renderer.rect(
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
