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

    let mut game = GameRenderer::new(
        initialize_map(),
        4,
        8,
        Player::new((10, 10), 10),
        initialize_map_colliders()
    );

    let mut window = Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear((120, 120, 120));

        input_handle(&window, &mut game);

        game.render_map(&mut renderer, 100, 200);
        game.render_player(&mut renderer);
    }
}

fn input_handle(window: &Window, game: &mut GameRenderer) {
    let player_speed = 3;

    if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
        game.player.position.y -= player_speed;
    }
    if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
        game.player.position.x -= player_speed;
    }
    if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
        game.player.position.y += player_speed;
    }
    if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
        game.player.position.x += player_speed;
    }
}

fn initialize_map() -> Vec<Block> {
    let w = Block::Wall;
    let e = Block::Empty;

    #[rustfmt::skip]
    let map = vec![
        w, e, e, e, 
        e, w, w, w,
        e, e, e, e,
        e, e, e, e,
        e, e, e, e,
        e, e, e, e,
        e, e, e, e,
        e, e, e, w,
    ];

    return map;
}
