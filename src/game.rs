use std::f32::consts::PI;

use crate::Window;
use crate::renderer::{to_color, MAP_WIDTH, MAP_HEIGHT, gamespace_to_screenspace, MAP_WIDTH_COUNT, MAP_HEIGHT_COUNT};
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub player: Player,
    pub colliders: Vec<SquareCollider>,
}

impl GameRenderer {
    pub fn new(
        map: Vec<Block>,
        player: Player,
        colliders: Vec<SquareCollider>,
    ) -> Self {
        Self {
            map,
            player,
            colliders,
        }
    }

    pub fn render_map(
        &mut self,
        render_handle: &mut WindowRenderer
    ) {
        
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
    }

    pub fn render_player_vission(&mut self, render_handle: &mut WindowRenderer) {
        let fov = PI/6.0;
        let depth = 8.0;

        for i in 0..1 {

        let mut distance = 0.0;    
        let ray_angle = (self.player.direction - fov / 2.0) + (i as f32 / WIDTH as f32) * fov;

        let mut hit_wall = false;

        while distance < depth && !hit_wall {
            distance += 0.1;
            
            let test_x = (self.player.position.x as f32 + ray_angle.sin() * distance) as i32;
            let test_y = (self.player.position.y as f32 + ray_angle.cos() * distance) as i32;

            // test om ray er out of bounds
            if test_x < 0 || test_x <= MAP_WIDTH_COUNT as i32 || test_y < 0 || test_y >= MAP_HEIGHT_COUNT as i32 {
                hit_wall = true; // sæt distance til max depth
                distance = depth;
            }
            else {
                if self.map[(test_y * MAP_WIDTH_COUNT as i32 + test_x) as usize] == Block::Wall {
                    hit_wall = true;
                    println!("wall is hit");
                }
            }

        }
            
            let player_position = gamespace_to_screenspace(self.player.position);
            let x = (player_position.x as f32 + self.player.size as f32 * 0.5) as f32;
            let y = (player_position.y as f32 + self.player.size as f32 * 0.5) as f32;

            let line = Line::new(
                (x, y),
                (
                    x - (self.player.direction.sin() * distance * MAP_HEIGHT_COUNT as f32) as f32,
                    y - (self.player.direction.cos() * distance * MAP_WIDTH_COUNT as f32) as f32,
                ),
            );
            render_handle.line(&line, (255,255,255));
        }

    }

    pub fn move_player(&mut self, window: &Window) {
        let player_speed = 0.15;
        let rotation_speed = 0.1;
        let direction_vec = vec![self.player.direction.sin(), self.player.direction.cos()];

        

        window
            .get_keys_pressed(KeyRepeat::Yes)
            .iter()
            .for_each(|key| match key {
                Key::W => {
                    self.player.position.x += (player_speed * direction_vec[0]);
                    self.player.position.y += (player_speed * direction_vec[1]);
                }
                Key::S => {
                    self.player.position.x -= (player_speed * direction_vec[0]);
                    self.player.position.y -= (player_speed * direction_vec[1]);
                }
                Key::A => {
                    self.player.direction -= rotation_speed;
                }
                Key::D => {
                    self.player.direction += rotation_speed;
                }
                _ => {}
            });
    }

    pub fn render_player(&mut self, render_handle: &mut WindowRenderer) {
        let player_screen_position = gamespace_to_screenspace(self.player.position);

        let player_map_sprite = Square::from((
            self.player.size,
            self.player.size,
            (player_screen_position.x, player_screen_position.y),
        ));
        render_handle.rect(player_map_sprite, Color::from((200, 0, 0)).rgb);
    }

    pub fn draw_walls_3d(&mut self, render_handle: &mut WindowRenderer) { 
        let fov = PI/6.0;
        let depth = 8.0;

        //println!("pos{:?} dir: {:?}", self.player.position, self.player.direction);
        for x in 0..WIDTH {

            let mut distance = 0.0;   
            let mut hit_wall = false;

            let ray_angle = (self.player.direction - fov / 2.0) + (x as f32 / WIDTH as f32) * fov;

            while distance < depth && !hit_wall {
                distance += 0.1;
            
                let test_x = (self.player.position.x as f32 + ray_angle.sin() * distance) as i32;
                let test_y = (self.player.position.y as f32 + ray_angle.cos() * distance) as i32;

            // test om ray er out of bounds
            if test_x < 0 || test_x >= MAP_WIDTH_COUNT as i32 || test_y < 0 || test_y >= MAP_HEIGHT_COUNT as i32 {
                hit_wall = true; // sæt distance til max depth
                distance = depth;

            }
            else {
                // ray er indenfor dypden, så vi tjekker om den rammer en mur
                if self.map[(test_y * MAP_WIDTH_COUNT as i32 + test_x) as usize] == Block::Wall {
                    hit_wall = true;
                }
            }

        }

            // calculate distance to ceiling and floor
            let mut ceiling = (HEIGHT as f32 / 2.0) - (HEIGHT as f32 / distance);
            let mut floor = HEIGHT as f32 - ceiling;

            for y in 0..HEIGHT {
                if y < ceiling as usize {
                    render_handle.pixel(
                        (x as f32, y as f32),
                        (0, 0, 0),
                    );
                } else if y > ceiling as usize && y <= floor as usize {
                    render_handle.pixel(
                        (x as f32, y as f32),
                        (255,255,255),
                    );
                } else {
                    render_handle.pixel(
                        (x as f32, y as f32),
                        (20, 20, 20),
                    );
                }
            }   
        }
    }
}


pub fn initialize_map_colliders() -> Vec<SquareCollider> {
    let mut collider_array: Vec<SquareCollider> = vec![];
    collider_array.push(SquareCollider::from(((0.0, 0.0), (100.0, 20.0))));
    return collider_array;
}

pub fn initialize_map() -> Vec<Block> {
    let w = Block::Wall;
    let e = Block::Empty;

    #[rustfmt::skip]
    let map = vec![
        w, w, w, w, w, w, w, w, 
        w, e, e, e, e, e, e, w,
        w, e, e, e, w, w, e, w,
        w, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, w,
        w, w, w, w, w, w, w, w,
    ];

    return map;
}
