pub struct SquareHitbox {
    x: f32,
    y: f32,
    side_length: f32,
}

impl SquareHitbox {
    pub fn intersects(player_hitbox: &Square, wall_hitbox: &Square) -> bool {
        let corners = [
            (self.x, self.y),
            (self.x + self.side_length, self.y),
            (self.x, self.y + self.side_length),
            (self.x + self.side_length, self.y + self.side_length),
        ];
        for &(x, y) in corners.iter() {
            if x >= other.x
                && x <= other.x + other.side_length
                && y >= other.y
                && y <= other.y + other.side_length
            {
                return true;
            }
        }
        return false;
    }
}

pub fn check_squares(HitboxArr: &[Square], playerHitbox: &Square) -> bool {
    for square in HitboxArr {
        if square.intersects(playerHitbox) {
            return true;
        }
    }
    return false;
}
