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

pub struct Enemy {

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

    fn check_collision(self) {
       
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
                    self.player.position.x += (player_speed * direction_vec[0]);
                    self.player.position.y += (player_speed * direction_vec[1]);
                }
                Key::S => {
                    self.player.position.x -= (player_speed * direction_vec[0]);
                    self.player.position.y -= (player_speed * direction_vec[1]);
                }
                //strafe
                Key::A => {
                    self.player.position.x -= (player_speed * direction_vec[1]);
                    self.player.position.y += (player_speed * direction_vec[0]);
                }
                Key::D => {
                    self.player.position.x += (player_speed * direction_vec[1]);
                    self.player.position.y -= (player_speed * direction_vec[0]);
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

pub fn check_collision() {

/*
let player_x = self.player.position.x;
let player_y = self.player.position.y;
let player_size = self.player.hitbox_size;
 */

let player_x = 5;
let player_y = 5;
let player_size = 10;


struct Corner {
    x: usize,
    y: usize,
}

let PlayerTopLeft = Corner{x: player_x - player_size/2 , y: player_y - player_size/2};
let PlayerTopRight = Corner{x: player_x - player_size/2 , y: player_y + player_size/2};
let PlayerBotLeft = Corner{x: player_x + player_size/2 , y: player_y - player_size/2};
let PlayerBotRight = Corner{x: player_x + player_size/2 , y: player_y + player_size/2};



let L = 10;
let H = 10;
let pos_x = 5;
let pos_y = 5;
//let test_box: Square = Square::new(L, H, ({pos_x},{pos_y}));

let MapTopLeft = Corner{x: pos_x - L/2 , y: pos_y - H/2};
let MapTopRight = Corner{x: pos_x - L/2 , y: pos_y + H/2};
let MapBotLeft = Corner{x: pos_x + L/2 , y: pos_y - H/2};
let MapBotRight = Corner{x: pos_x + L/2 , y: pos_y + H/2};

if(PlayerTopLeft.x <= MapBotRight.x && PlayerTopLeft.x >= MapTopLeft.x && PlayerTopLeft.y <= MapBotRight.y && PlayerTopLeft.y >= MapTopLeft.y){
    println!("tl2")
    //return true
  }
  
  if(PlayerBotRight.x <= MapBotRight.x && PlayerBotRight.x >= MapBotLeft.x && PlayerBotRight.y <= MapBotRight.y && PlayerBotRight.y >= MapBotLeft.y){
    println!("br2")
    //return true
  }
  
  if(PlayerBotLeft.x <= MapBotRight.x && PlayerBotLeft.x >= MapTopLeft.x && PlayerBotLeft.y <= MapBotRight.y && PlayerBotLeft.y >= MapTopLeft.y){
    println!("bl2")
    //return true
  }
  
  if(PlayerTopRight.x <= MapBotRight.x && PlayerTopRight.x >= MapTopLeft.x && PlayerTopRight.y <= MapBotRight.y && PlayerTopRight.y >= MapTopLeft.y){
    println!("tr2")
    //return true
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
