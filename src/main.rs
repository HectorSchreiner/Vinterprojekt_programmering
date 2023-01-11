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
    // let mut game: Game = Game {
    //     renderer: &mut renderer
    // };

    let mut game = GameRenderer::new(
        initialize_map(),
        4,
        8,
        Player::new((50, 100), 10),
        initialize_map_colliders(),
    );

    let mut window = Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear((120, 120, 120));
        game.draw_walls_3d(&mut renderer);
        game.render_map(&mut renderer, 100, 200);
        game.render_player(&mut renderer);
        game.move_player(&window);
        game.render_player_vision(&mut renderer);
    }
}
