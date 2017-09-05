mod aabb;
mod moving_object;
mod ledge_grabbing;
mod physics_nc;
mod quad_tree;

pub use self::aabb::AABB;
pub use self::moving_object::MovingObject;
pub use self::ledge_grabbing::LedgeGrabbing;

use super::Vector2;
use std::time::Duration;

pub fn seconds(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + (dur.subsec_nanos() as f64 / 1000000000.0)
}

pub fn lerp(v1: &Vector2, v2: &Vector2, by: f64) -> Vector2 {
    v1 * by + v2 * (1.0 - by)
}

pub fn round_vector(mut v: Vector2) -> Vector2 {
    v.x = v.x.round();
    v.y = v.y.round();
    v
}
