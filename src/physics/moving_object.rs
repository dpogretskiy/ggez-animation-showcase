use super::*;
use level::Terrain;
use std::time::Duration;
use std::cmp;
use debug::Debug;

#[derive(Debug, Clone)]
pub struct MovingObject {
    pub old_position: Vector2,
    pub position: Vector2,

    pub old_accel: Vector2,
    pub accel: Vector2,

    pub old_velocity: Vector2,
    pub velocity: Vector2,

    pub aabb: AABB,

    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,

    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,

    pub was_on_ground: bool,
    pub on_ground: bool,

    pub on_platform: bool,

    pub was_at_ceiling: bool,
    pub at_ceiling: bool,

    pub cannot_go_left_frames: usize,
    pub cannot_go_right_frames: usize,

    pub frames_from_jump_start: isize,
}

impl MovingObject {
    pub fn new(position: Vector2, aabb: AABB) -> MovingObject {
        MovingObject {
            old_position: position.clone(),
            position: position.clone(),
            old_accel: Vector2::new(0.0, 0.0),
            accel: Vector2::new(0.0, 0.0),
            old_velocity: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            aabb: aabb,
            pushed_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            pushes_right_wall: false,
            was_on_ground: false,
            on_ground: false,
            on_platform: false,
            was_at_ceiling: false,
            at_ceiling: false,
            cannot_go_left_frames: 0,
            cannot_go_right_frames: 0,
            frames_from_jump_start: 0,
        }
    }

    pub fn update_physics(&mut self, time: &Duration, terrain: &Terrain) {
        self.old_position = self.position;
        self.old_velocity = self.velocity;
        self.old_accel = self.accel;

        self.was_on_ground = self.on_ground;
        self.was_at_ceiling = self.at_ceiling;
        self.pushed_left_wall = self.pushes_left_wall;
        self.pushed_right_wall = self.pushes_right_wall;

        self.velocity += self.accel * seconds(time);
        self.position += self.velocity * seconds(time);

        let mut ground_y = 0.0;
        let mut ceiling_y = 0.0;
        let mut right_wall_x = 0.0;
        let mut left_wall_x = 0.0;

        self.on_platform = false;

        if self.velocity.y <= 0.0 && self.has_ground(&mut ground_y, terrain) {
            self.position.y = ground_y + self.aabb.half_size().y - self.aabb.offset.y;
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }

        if self.velocity.x <= 0.0 && self.collides_with_left_wall(&mut left_wall_x, terrain) {
            if self.old_position.x - self.aabb.half_size().x + self.aabb.offset.x >= left_wall_x {
                self.position.x = left_wall_x + self.aabb.half_size().x - self.aabb.offset.x;
                self.pushes_left_wall = true;
            };
            self.velocity.x = self.velocity.x.max(0.0);
            self.accel.x = self.accel.x.max(0.0);
        } else {
            self.pushes_left_wall = false;
        }

        if self.velocity.x >= 0.0 && self.collides_with_right_wall(&mut right_wall_x, terrain) {
            if self.old_position.x + self.aabb.half_size().x + self.aabb.offset.x <= right_wall_x {
                self.position.x = right_wall_x - self.aabb.half_size().x - self.aabb.offset.x;
                self.pushes_right_wall = true;
            }
            self.velocity.x = self.velocity.x.min(0.0);
            self.accel.x = self.accel.x.min(0.0);
        } else {
            self.pushes_right_wall = false;
        }


        if self.velocity.y >= 0.0 && self.has_ceiling(&mut ceiling_y, terrain) {
            self.position.y = ceiling_y - self.aabb.half_size().y - self.aabb.offset.y - 1.0;
            self.velocity.y = 0.0;
            self.at_ceiling = true;
        } else {
            self.at_ceiling = false;
        }

        self.aabb.center = self.position + self.aabb.offset;

        Debug::detect_teleporting(&self);
    }

    pub fn has_ground(&mut self, ground_y: &mut f64, terrain: &Terrain) -> bool {
        let old_center = self.old_position + self.aabb.offset;
        let center = self.position + self.aabb.offset;
        let old_bottom_left =
            round_vector(old_center - self.aabb.half_size() + Vector2::new(1.0, -1.0));
        let new_bottom_left =
            round_vector(center - self.aabb.half_size() + Vector2::new(1.0, -1.0));
        let end_y = terrain.get_tile_y_at_point(new_bottom_left.y);
        let beg_y = cmp::max(terrain.get_tile_y_at_point(old_bottom_left.y) - 1, end_y);
        let dist = cmp::max((end_y - beg_y).abs(), 1);
        let mut tile_index_x;
        for tile_index_y in (end_y..beg_y + 1).rev() {
            let bottom_left = lerp(
                &new_bottom_left,
                &old_bottom_left,
                (end_y - tile_index_y).abs() as f64 / dist as f64,
            );

            let bottom_right = Vector2::new(
                bottom_left.x + self.aabb.half_size().x * 2.0 - 2.0,
                bottom_left.y,
            );
            let mut checked_tile = bottom_left.clone();
            'inner: loop {
                checked_tile.x = checked_tile.x.min(bottom_right.x);
                tile_index_x = terrain.get_tile_x_at_point(checked_tile.x);
                *ground_y = tile_index_y as f64 * terrain.tile_size + terrain.tile_size / 2.0 +
                    terrain.position.y;
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    self.on_platform = false;
                    return true;
                } else if terrain.is_one_way_platform(tile_index_x, tile_index_y) &&
                           (checked_tile.y - *ground_y).abs() <=
                               MovingObject::PLATFORM_THRESHOLD + self.old_position.y -
                                   self.position.y
                {
                    self.on_platform = true;
                };
                if checked_tile.x >= bottom_right.x {
                    if self.on_platform {
                        return true;
                    };
                    break 'inner;
                }
                checked_tile.x += terrain.tile_size;
            }
        }
        false
    }

    pub fn has_ceiling(&self, ceiling_y: &mut f64, terrain: &Terrain) -> bool {
        let center = self.position + self.aabb.offset;
        let old_center = self.old_position + self.aabb.offset;
        *ceiling_y = 0.0;
        let old_top_right =
            round_vector(old_center + self.aabb.half_size() + Vector2::new(-1.0, 1.0));
        let new_top_right = round_vector(center + self.aabb.half_size() + Vector2::new(-1.0, 1.0));
        let end_y = terrain.get_tile_y_at_point(new_top_right.y);
        let beg_y = cmp::min(terrain.get_tile_y_at_point(old_top_right.y) + 1, end_y);
        let dist = cmp::max((end_y - beg_y).abs(), 1);
        let mut tile_index_x;
        for tile_index_y in beg_y..end_y + 1 {
            let top_right = lerp(
                //these 2 should be the opposite way...?
                &old_top_right,
                &new_top_right,
                ((end_y - tile_index_y).abs() as f64 / dist as f64),
            );
            let top_left = Vector2::new(
                top_right.x - self.aabb.half_size().x * 2.0 + 2.0,
                top_right.y,
            );
            let mut checked_tile = top_left.clone();
            loop {
                checked_tile.x = checked_tile.x.min(top_right.x);
                tile_index_x = terrain.get_tile_x_at_point(checked_tile.x);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *ceiling_y = tile_index_y as f64 * terrain.tile_size - terrain.tile_size / 2.0 +
                        terrain.position.y;
                    return true;
                }
                if checked_tile.x >= top_right.x {
                    break;
                }
                checked_tile.x += terrain.tile_size;
            }
        }
        false
    }

    pub fn collides_with_left_wall(&self, wall_x: &mut f64, terrain: &Terrain) -> bool {
        let center = self.position + self.aabb.offset;
        let old_center = self.old_position + self.aabb.offset;
        *wall_x = 0.0;
        let old_bottom_left =
            round_vector(old_center - self.aabb.half_size() - Vector2::new(1.0, 0.0));
        let new_bottom_left = round_vector(center - self.aabb.half_size() - Vector2::new(1.0, 0.0));
        let mut tile_index_y;
        let end_x = terrain.get_tile_x_at_point(new_bottom_left.x);
        let beg_x = cmp::max(terrain.get_tile_x_at_point(old_bottom_left.x) - 1, end_x);
        let dist = cmp::max((end_x - beg_x).abs(), 1);
        for tile_index_x in (end_x..beg_x + 1).rev() {
            let bottom_left = lerp(
                &new_bottom_left,
                &old_bottom_left,
                (end_x - tile_index_x).abs() as f64 / dist as f64,
            );
            let top_left = bottom_left + Vector2::new(0.0, self.aabb.half_size().y * 2.0);
            let mut checked_tile = bottom_left;
            loop {
                checked_tile.y = checked_tile.y.min(top_left.y);
                tile_index_y = terrain.get_tile_y_at_point(checked_tile.y);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *wall_x = tile_index_x as f64 * terrain.tile_size + terrain.tile_size / 2.0 +
                        terrain.position.x;
                    return true;
                }
                if checked_tile.y >= top_left.y {
                    break;
                }
                checked_tile.y += terrain.tile_size;
            }
        }
        false
    }

    pub fn collides_with_right_wall(&self, wall_x: &mut f64, terrain: &Terrain) -> bool {
        let center = self.position + self.aabb.offset;
        let old_center = self.old_position + self.aabb.offset;
        *wall_x = 0.0;
        let old_bottom_right =
            round_vector(
                old_center + Vector2::new(self.aabb.half_size().x, -self.aabb.half_size().y) +
                    Vector2::new(1.0, 0.0),
            );
        let new_bottom_right =
            round_vector(
                center + Vector2::new(self.aabb.half_size().x, -self.aabb.half_size().y) +
                    Vector2::new(1.0, 0.0),
            );
        let end_x = terrain.get_tile_x_at_point(new_bottom_right.x);
        let beg_x = cmp::min(terrain.get_tile_x_at_point(old_bottom_right.x) + 1, end_x);
        let dist = cmp::max((end_x - beg_x).abs(), 1);
        let mut tile_index_y;
        for tile_index_x in beg_x..end_x + 1 {
            let bottom_right = lerp(
                &new_bottom_right,
                &old_bottom_right,
                (end_x - tile_index_x).abs() as f64 / dist as f64,
            );
            let top_right = bottom_right + Vector2::new(0.0, self.aabb.half_size().y * 2.0);
            let mut checked_tile = bottom_right;
            loop {
                checked_tile.y = checked_tile.y.min(top_right.y);
                tile_index_y = terrain.get_tile_y_at_point(checked_tile.y);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *wall_x = tile_index_x as f64 * terrain.tile_size - terrain.tile_size / 2.0 +
                        terrain.position.x;
                    return true;
                }
                if checked_tile.y >= top_right.y {
                    break;
                }
                checked_tile.y += terrain.tile_size;
            }
        }
        false
    }

    pub const PLATFORM_THRESHOLD: f64 = 2.0;
}
