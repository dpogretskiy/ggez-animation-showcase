mod aabb;
mod moving_object;
mod ledge_grabbing;
mod double_jumping;
mod quad_tree;
pub mod world;

pub use self::aabb::*;
pub use self::moving_object::*;
pub use self::ledge_grabbing::*;
pub use self::double_jumping::*;

use super::Vector2;
use std::time::Duration;

pub fn seconds(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + (dur.subsec_nanos() as f64 / 1000000000.0)
}

pub fn lerp(v1: &Vector2, v2: &Vector2, by: f64) -> Vector2 {
    v1 * (1.0 - by) + v2 * by
}

pub fn round_vector(mut v: Vector2) -> Vector2 {
    v.x = v.x.round();
    v.y = v.y.round();
    v
}
