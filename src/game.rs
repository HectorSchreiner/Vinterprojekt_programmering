use std::f32::consts::PI;

use crate::renderer::{MAP_HEIGHT_COUNT, MAP_WIDTH_COUNT};
use crate::shapes::*;
use crate::Window;
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

pub trait CanMove {
    fn move_enemy() {
        // move enemy ...
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EnemyType {
    Ghost,
    Robot,
}

impl CanMove for EnemyType {
    fn move_enemy() {}
}

pub struct Enemy {
    position: Position2D,
    enemy_type: EnemyType,
    health: f32,
    direction: f32,
}

impl Enemy {
    pub fn new<T: Into<Position2D>>(
        position: T,
        enemy_type: EnemyType,
        health: f32,
        direction: f32,
    ) -> Self {
        Self {
            position: position.into(),
            enemy_type,
            health,
            direction,
        }
    }

    pub fn rotate_enemy(&mut self, game_renderer: GameRenderer) {
        let turn_speed = 0.1;
        let position_diff: (f32, f32) = (
            game_renderer.player.position.x - self.position.x,
            game_renderer.player.position.y - self.position.y,
        );

        // smart vector regning
        if position_diff.0 * self.direction.sin() > position_diff.1 * self.direction.cos() {
            // turn right
            self.direction -= turn_speed;
        } else {
            // turn left
            self.direction += turn_speed;
        }
    }

    pub fn move_enemy(&mut self) {
        let move_speed = 0.2;

        self.position.x += self.direction.sin() * move_speed;
        self.position.y += self.direction.cos() * move_speed;
    }

    pub fn display_enemy(&mut self) {}
}

pub struct GameRenderer {
    pub map: Vec<Block>,
    pub player: Player,
}

impl GameRenderer {
    pub fn new(map: Vec<Block>, player: Player) -> Self {
        Self { map, player }
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
                side_length: 1.0,
            };
            // hvis rammer return true
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                collides = true;
            }
        }

        // tilbage
        if direction == 2 {
            let future_pos: SquareHitbox = SquareHitbox {
                x: self.player.position.x - (player_speed * direction_vec[0]),
                y: self.player.position.y - (player_speed * direction_vec[1]),
                side_length: 1.0,
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
                side_length: 1.0,
            };
            // hvis rammer return false
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
                collides = true;
            }
        }

        // højre
        if direction == 4 {
            let future_pos: SquareHitbox = SquareHitbox {
                x: self.player.position.x + (player_speed * direction_vec[1]),
                y: self.player.position.y - (player_speed * direction_vec[0]),
                side_length: 1.0,
            };
            // hvis rammer return false
            if check_player_collision(&wall_hitbox_arr, &future_pos) {
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
                        self.player.position.x += player_speed * direction_vec[0];
                        self.player.position.y += player_speed * direction_vec[1];
                    }
                }
                Key::S => {
                    if !self.check_player_move(2, player_speed) {
                        self.player.position.x -= player_speed * direction_vec[0];
                        self.player.position.y -= player_speed * direction_vec[1];
                    }
                }
                //strafe
                Key::A => {
                    if !self.check_player_move(3, player_speed) {
                        self.player.position.x -= player_speed * direction_vec[1];
                        self.player.position.y += player_speed * direction_vec[0];
                    }
                }
                Key::D => {
                    if !self.check_player_move(4, player_speed) {
                        self.player.position.x += player_speed * direction_vec[1];
                        self.player.position.y -= player_speed * direction_vec[0];
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
        let fov = PI / 4.0;
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
                if test_x < 0
                    || test_x >= MAP_WIDTH_COUNT as i32
                    || test_y < 0
                    || test_y >= MAP_HEIGHT_COUNT as i32
                {
                    hit_wall = true; // sæt distance til max depth
                    distance = depth;
                } else {
                    // ray er indenfor dypden, så vi tjekker om den rammer en mur
                    if self.map[(test_y * MAP_WIDTH_COUNT as i32 + test_x) as usize] == Block::Wall
                    {
                        hit_wall = true;
                    }
                }
            }

            // calculate distance to ceiling and floor
            let ceiling = (HEIGHT as f32 / 2.0) - (HEIGHT as f32 / distance);
            let floor = HEIGHT as f32 - ceiling;
            let dist_color = (255.0 - (255.0 * distance / depth)) as u8;
            let wall_color = (dist_color, dist_color, dist_color);

            for y in 0..HEIGHT {
                // render tom
                if y < ceiling as usize {
                    render_handle.pixel((x as f32, y as f32), (0, 0, 0));

                    // render mur
                } else if y > ceiling as usize && y <= floor as usize {
                    render_handle.pixel((x as f32, y as f32), wall_color);

                    // render gulv
                } else {
                    render_handle.pixel((x as f32, y as f32), (20, 20, 20));
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
        w, e, e, e, e, w, w, e, e, e, e, e, e, e, e, w,
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
        let x = array_index as i32 % MAP_WIDTH_COUNT;
        let y = (array_index as f32 / MAP_HEIGHT_COUNT as f32).floor() as i32;

        Position2D {
            x: x as f32,
            y: y as f32,
        }
    }

    for (i, item) in map_array.iter().enumerate() {
        if *item == Block::Wall {
            hitbox_array.push(SquareHitbox {
                x: position(i).x + 0.5,
                y: position(i).y + 0.5,
                side_length: 1.0,
            })
        }
    }
    hitbox_array
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SquareHitbox {
    x: f32,
    y: f32,
    side_length: f32,
}

impl SquareHitbox {
    // tjekker om self collider med otherHitbox
    pub fn self_intersects(&self, other_hitbox: &SquareHitbox) -> bool {
        let corners = [
            (self.x, self.y),
            (self.x + self.side_length, self.y),
            (self.x, self.y + self.side_length),
            (self.x + self.side_length, self.y + self.side_length),
        ];
        for &(x, y) in corners.iter() {
            if x >= other_hitbox.x
                && x <= other_hitbox.x + other_hitbox.side_length
                && y >= other_hitbox.y
                && y <= other_hitbox.y + other_hitbox.side_length
            {
                return true;
            }
        }
        false
    }
}

// tjekker om player collider med en mur i wall array
pub fn check_player_collision(
    wall_hitbox_arr: &[SquareHitbox],
    player_hitbox: &SquareHitbox,
) -> bool {
    for square in wall_hitbox_arr {
        if square.self_intersects(player_hitbox) {
            return true;
        }
    }
    false
}
