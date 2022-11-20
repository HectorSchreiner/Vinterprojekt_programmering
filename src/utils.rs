pub const PI: f32 = 3.14159265;
pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}