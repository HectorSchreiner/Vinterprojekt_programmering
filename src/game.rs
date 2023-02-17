use std::array;
use std::f32::consts::PI;

use crate::Window;
use crate::renderer::{to_color, MAP_WIDTH, MAP_HEIGHT, MAP_WIDTH_COUNT, MAP_HEIGHT_COUNT};
use crate::{renderer, shapes::*};
use crate::{WindowRenderer, HEIGHT, WIDTH};
use minifb::*;


#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Wall,
    Empty,
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
}

impl GameRenderer {
    pub fn new(
        map: Vec<Block>,
        player: Player,
    ) -> Self {
        Self {
            map,
            player,
        }
    }

    pub fn render_map(
        &mut self,
        render_handle: &mut WindowRenderer
    ) {
        
    }

    // tjekker om player kan bevæge sig og returnerer enten sandt eller falsk
    pub fn check_player_move(&mut self, direction: i32, player_speed: f32) -> bool {
        let direction_vec = vec![self.player.direction.sin(), self.player.direction.cos()];

        let mut collides = false;

        let wall_hitbox_arr = initialize_map_hitboxes();
        // frem
        if direction == 1 {
            let future_pos: SquareHitbox = SquareHitbox { 
                x: self.player.position.x + (player_speed * direction_vec[0]), 
                y: self.player.position.y + (player_speed * direction_vec[1]), 
                sideLength: 1.0 
            };
            // hvis rammer return true
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                println!("collision");
                collides = true;
            }
        }

        // tilbage
        if direction == 2 {
            let future_pos: SquareHitbox = SquareHitbox { 
                x: self.player.position.x - (player_speed * direction_vec[0]), 
                y: self.player.position.y - (player_speed * direction_vec[1]), 
                sideLength: 1.0 
            };
            // hvis rammer return false
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                println!("collision");

                collides = true;
            }
        }

        // venstre
        if direction == 3 {
            let future_pos: SquareHitbox = SquareHitbox { 
                x: self.player.position.x - (player_speed * direction_vec[1]), 
                y: self.player.position.y + (player_speed * direction_vec[0]), 
                sideLength: 1.0 
            };
            // hvis rammer return false
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                println!("collision");

                collides = true;
            }
        }

        // højre
        if direction == 4 {
            let future_pos: SquareHitbox = SquareHitbox { 
                x: self.player.position.x + (player_speed * direction_vec[1]), 
                y: self.player.position.y - (player_speed * direction_vec[0]), 
                sideLength: 1.0 
            };
            // hvis rammer return false
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                println!("collision");
                collides = true;
            }
        }

        collides
    }

    pub fn move_player(&mut self, window: &Window) {
        let player_speed = 0.15;
        let rotation_speed = 0.1;
        let direction_vec = vec![self.player.direction.sin(), self.player.direction.cos()];

        window
            .get_keys_pressed(KeyRepeat::Yes)
            .iter()
            .for_each(|key| match key {
                //forward backward
                Key::W => {
                    if !self.check_player_move(1, player_speed) {
                    self.player.position.x += (player_speed * direction_vec[0]);
                    self.player.position.y += (player_speed * direction_vec[1]);
                    }
                    
                }
                Key::S => {
                    if !self.check_player_move(2, player_speed) {
                    self.player.position.x -= (player_speed * direction_vec[0]);
                    self.player.position.y -= (player_speed * direction_vec[1]);
                    }
                }
                //strafe
                Key::A => {
                    if !self.check_player_move(3, player_speed) {
                    self.player.position.x -= (player_speed * direction_vec[1]);
                    self.player.position.y += (player_speed * direction_vec[0]);
                    }
                }
                Key::D => {
                    if !self.check_player_move(4, player_speed) {
                    self.player.position.x += (player_speed * direction_vec[1]);
                    self.player.position.y -= (player_speed * direction_vec[0]);
                    }
                }

                //rotate
                Key::J => {
                    self.player.direction -= rotation_speed;
                }
                Key::K => {
                    self.player.direction += rotation_speed;
                }
                _ => {}
            });
    }

    pub fn draw_walls_3d(&mut self, render_handle: &mut WindowRenderer) { 
        let fov = PI/6.0;
        let depth = 16.0;

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
            let dist_color = (255.0 - (255.0 * distance / depth)) as u8;
            let mut wall_color = (dist_color, dist_color, dist_color);

            for y in 0..HEIGHT {

                // render tom 
                if y < ceiling as usize {
                    render_handle.pixel(
                        (x as f32, y as f32),
                        (0, 0, 0),
                    );

                    // render mur
                } else if y > ceiling as usize && y <= floor as usize {
                    render_handle.pixel(
                        (x as f32, y as f32),
                        wall_color,
                    );

                    // render gulv
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


pub fn initialize_map() -> Vec<Block> {
    let w = Block::Wall;
    let e = Block::Empty;

    #[rustfmt::skip]
    let map = vec![
        w, w, w, w, w, w, w, w, w, w, w, w ,w ,w ,w, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w, 
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w,
        w, w, w, w, w, w, w, w, w, w, w, w, w, w, w, w,
    ];

    map
}

pub fn initialize_map_hitboxes() -> Vec<SquareHitbox> {
    let map_array: Vec<Block> = initialize_map();
    let mut hitbox_array: Vec<SquareHitbox> = vec![];

    fn position(array_index: usize) -> Position2D {
        let x = (array_index as i32 % MAP_WIDTH_COUNT);
        let y = (array_index as i32 / MAP_HEIGHT_COUNT);

        Position2D { x: x as f32, y: y as f32 }
    }
    
    for (i, item) in map_array.iter().enumerate() {
        if *item == Block::Wall {
            hitbox_array.push(SquareHitbox { x: position(i).x, y: position(i).y, sideLength: 1.0 })
        }
    }

    return hitbox_array;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SquareHitbox {
    x: f32,
    y: f32,
    sideLength: f32,
}

impl SquareHitbox {

    // tjekker om self collider med otherHitbox
    pub fn self_intersects(&self, otherHitbox: &SquareHitbox) -> bool {
        let corners = [
            (self.x, self.y),
            (self.x + self.sideLength, self.y),
            (self.x, self.y + self.sideLength),
            (self.x + self.sideLength, self.y + self.sideLength),
        ];
        for &(x, y) in corners.iter() {
            if x >= otherHitbox.x
                && x <= otherHitbox.x + otherHitbox.sideLength
                && y >= otherHitbox.y
                && y <= otherHitbox.y + otherHitbox.sideLength
            {
                return true;
            }
        }
        return false;
    }
}

// tjekker om player collider med en mur i wall array
pub fn check_player_collision(wallHitboxArr: &[SquareHitbox], playerHitbox: &SquareHitbox) -> bool {
    for square in wallHitboxArr {
        if square.self_intersects(playerHitbox) {
            return true;
        }
    }
    return false;
}

