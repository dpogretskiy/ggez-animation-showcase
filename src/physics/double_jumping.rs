use super::moving_object::MovingObject;
use level::Terrain;
use player::*;

pub struct DoubleJumping {
    available: bool,
}

impl DoubleJumping {
    pub fn new() -> DoubleJumping {
        DoubleJumping { available: false }
    }

    pub fn update(&mut self, mv: &mut MovingObject, pi: &PlayerInput) {
        if mv.was_on_ground && !mv.on_ground {
            self.available = true;
        }
        if mv.on_ground {
            self.available = false;
        }
    }

    pub fn double_jump(&mut self, mv: &mut MovingObject) {
        if self.available {
            mv.velocity.y = Player::JUMP_SPEED;
            self.available = false;
        }
    }
}
