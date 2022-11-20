use crate::utils::*;
use line_drawing::Bresenham;

pub struct Renderer {
    pub buffer: Vec<u32>,
}