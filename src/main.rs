pub use game::*;
use minifb::*;
pub use renderer::*;
pub use shapes::*;

mod game;
mod renderer;
mod shapes;

pub fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear((120, 120, 120));
        game::Game::render_map(&mut renderer, 5, 8, 20);
    }
}

fn input_handle(window: &Window) {
    if window.is_key_pressed(Key::W, minifb::KeyRepeat::No) {
        //something
    }
    if window.is_key_pressed(Key::A, minifb::KeyRepeat::No) {
        //something
    }
    if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
        //something
    }
    if window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
        //something
    }
}
