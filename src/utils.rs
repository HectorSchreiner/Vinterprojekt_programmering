pub const PI: f32 = 3.14159265;
pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

pub fn tuple_to_rgb(color: (u8, u8, u8)) -> u32 {
    let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    (r << 16) | (g << 8) | b
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
