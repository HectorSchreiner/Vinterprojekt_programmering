use crate::{initialize_map, renderer, shapes::*};
use crate::{Window};
use crate::{WindowRenderer, HEIGHT, WIDTH};
use minifb::*;

#[derive(Clone, Copy)]
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
        let grid_width = map_width / self.rows;
        let grid_height = map_height / self.cols;

        let wall_color = (40, 200, 200);
        let empty_color = (0, 0, 0);

        for y in 0..self.cols {
            for x in 0..self.rows {
                let color = match self.map[(x + y * self.rows) as usize] {
                    Block::Wall => wall_color,
                    Block::Empty => empty_color,
                };

                let cell =
                    Square::from((grid_width, grid_height, (x * grid_width, y * grid_height)));
                render_handle.rect(cell, (0, 0, 0));

                let spacing_cell = Square::from((
                    grid_width - spacing,
                    grid_height - spacing,
                    (spacing / 2 + x * grid_width, spacing / 2 + y * grid_height),
                ));

                render_handle.rect(spacing_cell, color);
            }
        }
    }

    fn check_collision(self) {
        // emil must do this  
        
        // Eksempel
        let x1 = self.colliders[0].top_left_corner.x;
        let y1 = self.colliders[0].top_left_corner.y;
        let x2 = self.colliders[0].bottom_right_corner.x;
        let y2 = self.colliders[0].bottom_right_corner.y;



    }

    pub fn move_player(mut self, window: &Window) {
        let player_speed = 3;



        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            self.player.position.y -= player_speed;
        }
        if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
            self.player.position.x -= player_speed;
        }
        if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
            self.player.position.y += player_speed;
        }
        if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
            self.player.position.x += player_speed;
        }


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
}

pub fn initialize_map_colliders() -> Vec<SquareCollider> {
    let mut collider_array: Vec<SquareCollider> = vec![];
    collider_array.push(SquareCollider::from(((0, 0), (100, 20))));
    return collider_array;
}
