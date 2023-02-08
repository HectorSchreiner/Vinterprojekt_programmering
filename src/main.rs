use image::open;
use minifb::*;

use crate::game::*;
use crate::renderer::*;
use crate::shapes::*;

mod game;
mod renderer;
mod shapes;

fn main() {
    let mut renderer = WindowRenderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    // let image = open("enemy_sprite.png").unwrap().into_rgba8();
    // let (width, height) = image.dimensions();
    // let image_data = image.into_raw();

    let mut game = GameRenderer::new(initialize_map(), Player::new((2.0, 2.0), 10));

    let mut window = Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let ghost = {
        const GHOST_BUF: &[u8] = include_bytes!("./enemy_sprite.png");
        image::load_from_memory(GHOST_BUF).unwrap().into_rgb8()
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear((120, 120, 120));
        game.draw_walls_3d(&mut renderer);
        game.move_player(&window);

        let width = 16;
        let height = 16;
        for i in 0..width as _ {
            for j in 0..height as _ {
                let [r, g, b] = &ghost.get_pixel(i as _, j as _).0;
                renderer.pixel(
                    Position2D {
                        x: i as _,
                        y: j as _,
                    },
                    (*r, *g, *b),
                );
            }
        }
    }
}
