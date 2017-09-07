use super::moving_object::MovingObject;
use player::*;

pub struct DoubleJumping {
    available: bool,
}

impl DoubleJumping {
    pub fn new() -> DoubleJumping {
        DoubleJumping { available: false }
    }

    //hardcore!
    pub fn enable(&mut self) {
        self.available = true;
    }

    pub fn double_jump(&mut self, mv: &mut MovingObject) {
        if self.available {
            // if mv.velocity.y >= 0.0 {
            //     mv.velocity.y += Player::JUMP_SPEED;
            // } else {
            mv.velocity.y = Player::JUMP_SPEED;
            // }
            self.available = false;
        }
    }
}
