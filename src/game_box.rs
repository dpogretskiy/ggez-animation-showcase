use physics::*;
use camera::*;
use level::*;
use player::Player;


use ggez::graphics::*;
use ggez::Context;

use std::time::Duration;
use super::Vector2;

use std::rc::Rc;

pub struct GameBox {
    pub mv: MovingObject,
    pub src: Rect,
    pub sprite: Rc<Image>,
}

impl GameBox {
    pub fn new(point: Vector2, ss: &Rc<Image>, src: Rect) -> GameBox {
        let aabb = AABB::new_full(
            point.clone(),
            Vector2::new(106.0, 106.0),
            Vector2::new(1.0, 1.0),
        );

        GameBox {
            mv: MovingObject::new(point, aabb),
            src: src.clone(),
            sprite: ss.clone(),
        }
    }

    pub fn draw_cam(&self, ctx: &mut Context, camera: &Camera) {
        let pos = Point::new(self.mv.position.x as f32, self.mv.position.y as f32);

        (*self.sprite)
            .draw_ex_camera(
                camera,
                ctx,
                DrawParam {
                    src: self.src.clone(),
                    dest: pos,
                    scale: Point::new(1.0, 1.0),
                    ..Default::default()
                },
            )
            .unwrap();
    }

    pub fn update(&mut self, time: &Duration, terrain: &Terrain) {
        if !self.mv.on_ground {
            let y_vel = Player::GRAVITY * seconds(&time) + self.mv.velocity.y;
            self.mv.velocity.y = y_vel.max(Player::MAX_FALLING_SPEED);
        };
        self.mv.update_physics(time, terrain);
    }
}
