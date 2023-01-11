use std::f32::consts::PI;

use crate::Window;
use crate::renderer::to_color;
use crate::{renderer, shapes::*};
use crate::{WindowRenderer, HEIGHT, WIDTH};
use minifb::*;


#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Wall,
    Empty,
}

pub struct SquareCollider {
    top_left_corner: Position2D,
    bottom_right_corner: Position2D,
}

impl<T: Into<Position2D>> From<(T, T)> for SquareCollider {
    fn from(coordinates: (T, T)) -> Self {
        Self {
            top_left_corner: coordinates.0.into(),
            bottom_right_corner: coordinates.1.into(),
        }
    }
}

pub struct Player {
    pub position: Position2D,
    pub size: usize,
    pub direction: f32,
}

impl Player {
    pub fn new<T: Into<Position2D>>(position: T, size: usize) -> Self {
        Self {
            position: position.into(),
            size,
            direction: 0.0,
        }
    }
}

pub struct GameRenderer {
    pub map: Vec<Block>,
    pub rows: usize,
    pub cols: usize,
    pub player: Player,
    pub colliders: Vec<SquareCollider>,
}

impl GameRenderer {
    pub fn new(
        map: Vec<Block>,
        rows: usize,
        cols: usize,
        player: Player,
        colliders: Vec<SquareCollider>,
    ) -> Self {
        Self {
            map,
            rows,
            cols,
            player,
            colliders,
        }
    }

    pub fn render_map(
        &mut self,
        render_handle: &mut WindowRenderer,
        map_width: usize,
        map_height: usize,
    ) {
        let spacing = 2;
        let grid_width: i32 = map_width as i32 / self.rows as i32;
        let grid_height: i32 = map_height as i32 / self.cols as i32;

        let wall_color = (40, 200, 200);
        let empty_color = (0, 0, 0);

        for y in 0..(self.cols as i32) {
            for x in 0..(self.rows as i32) {
                let color = match self.map[(x + y * self.rows as i32) as usize] {
                    Block::Wall => wall_color,
                    Block::Empty => empty_color,
                };

                let cell = Square::from((
                    grid_width as usize,
                    grid_height as usize,
                    (x * grid_width, y * grid_height),
                ));
                render_handle.rect(cell, (0, 0, 0));

                let spacing_cell = Square::from((
                    (grid_width - spacing) as usize,
                    (grid_height - spacing) as usize,
                    (spacing / 2 + x * grid_width, spacing / 2 + y * grid_height),
                ));

                render_handle.rect(spacing_cell, color);
            }
        }
    }

    fn check_collision(self) {
        // emil must do this

        // Eksempel
        for i in self.colliders {
            let x1 = i.top_left_corner.x;
            let y1 = i.top_left_corner.y;
            let x2 = i.bottom_right_corner.x;
            let y2 = i.bottom_right_corner.y;
        }

        let player_pos_x = self.player.position.x;
        let player_pos_y = self.player.position.y;
    }

    pub fn render_player_vision(&self, render_handle: &mut WindowRenderer) {
        let x = (self.player.position.x as f32 + self.player.size as f32 * 0.5) as i32;
        let y = (self.player.position.y as f32 + self.player.size as f32 * 0.5) as i32;
        let line_lenght = 20.0;
        let line = Line::new(
            (x, y),
            (
                x - (self.player.direction.sin() * line_lenght) as i32,
                y - (self.player.direction.cos() * line_lenght) as i32,
            ),
        );

        render_handle.line(&line, (250, 250, 0));
    }

    pub fn move_player(&mut self, window: &Window) {
        let player_speed = 3.0;
        let rotation_speed = 0.1;
        let direction_vec = vec![self.player.direction.sin(), self.player.direction.cos()];

        window
            .get_keys_pressed(KeyRepeat::Yes)
            .iter()
            .for_each(|key| match key {
                Key::W => {
                    self.player.position.x -= (player_speed * direction_vec[0]) as i32;
                    self.player.position.y -= (player_speed * direction_vec[1]) as i32;
                }
                Key::S => {
                    self.player.position.x += (player_speed * direction_vec[0]) as i32;
                    self.player.position.y += (player_speed * direction_vec[1]) as i32;
                }
                Key::A => {
                    self.player.direction += rotation_speed;
                }
                Key::D => {
                    self.player.direction -= rotation_speed;
                }
                _ => {}
            });
    }

    pub fn render_player(&self, render_handle: &mut WindowRenderer) {
        let player = &self.player;

        let player_map_sprite = Square::from((
            player.size,
            player.size,
            (player.position.x, player.position.y),
        ));
        render_handle.rect(player_map_sprite, Color::from((200, 0, 0)).rgb);
    }

    pub fn draw_walls_3d(&self, render_handle: &mut WindowRenderer) {    
        let map_height = 8;
        let map_width = 4;
    
        let depth = 16.0;
        let fov = PI/6.0;

        for x in 0..WIDTH {
            // for hver column, udregn den projiterede vinkel til world space
            let mut ray_angle = (self.player.direction - fov / 2.0) + (x as f32 / WIDTH as f32) * fov;
    
            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;
    
            let mut eye_x = ray_angle.sin();
            let mut eye_y = ray_angle.cos();
    
            while hit_wall == false && distance_to_wall < depth {
                distance_to_wall += 0.1;
    
                let mut test_x = (self.player.position.x as f32 + eye_x * distance_to_wall) as i32;
                let mut test_y = (self.player.position.y as f32 + eye_y * distance_to_wall) as i32;
    
                // test om ray er out of bounds
                if test_x < 0 || test_x >= map_width || test_y < 0 || test_y >= map_height {
                    hit_wall = true; // sæt distance til max depth
                    distance_to_wall = depth;
                } else {
                    {
                        if self.map[(test_y * map_width + test_x) as usize] == Block::Wall {
                            hit_wall = true;
                        }
                    }
                }
    
                // calculate distance to ceiling and floor
                let mut ceiling = (HEIGHT as f32 / 2.0) - (HEIGHT as f32 / distance_to_wall);
                let mut floor = HEIGHT as f32 - ceiling;
    
                for y in 0..HEIGHT {
                    if y < ceiling as usize {
                        render_handle.pixel(
                            (x as i32, y as i32),
                            (0, 0, 0),
                        );
                    } else if y > ceiling as usize && y <= floor as usize {
                        render_handle.pixel(
                            (x as i32, y as i32),
                            (255,255,255),
                        );
                    } else {
                        render_handle.pixel(
                            (x as i32, y as i32),
                            (20, 20, 20),
                        );
                    }
                }
            }
    }
}
}

pub fn initialize_map_colliders() -> Vec<SquareCollider> {
    let mut collider_array: Vec<SquareCollider> = vec![];
    collider_array.push(SquareCollider::from(((0, 0), (100, 20))));
    return collider_array;
}

pub fn initialize_map() -> Vec<Block> {
    let w = Block::Wall;
    let e = Block::Empty;

    #[rustfmt::skip]
    let map = vec![
        w, w, w, w, 
        w, e, e, w,
        w, e, e, w,
        w, e, w, w,
        w, e, e, w,
        w, e, e, w,
        w, e, e, w,
        w, w, w, w,
    ];

    return map;
}
