use minifb::*;
pub use renderer::*;
pub use utils::*;

mod utils;
mod renderer;

pub fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new("FluidWorks", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
