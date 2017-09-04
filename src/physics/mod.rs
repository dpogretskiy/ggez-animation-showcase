
use super::Vector2;
use super::level::Terrain;
use std::time::Duration;

#[derive(Debug)]
pub struct AABB {
    pub center: Vector2,
    pub half_size: Vector2,
}

impl AABB {
    pub fn new_full(center: Vector2, full_size: Vector2) -> AABB {
        AABB {
            center,
            half_size: full_size / 2.0,
        }
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        !(self.center.x - other.center.x > self.half_size.x + other.half_size.x) &&
            !(self.center.y - other.center.y > self.half_size.y + other.half_size.y)
    }
}

#[derive(Debug)]
pub struct MovingObject {
    pub old_position: Vector2,
    pub position: Vector2,

    pub old_velocity: Vector2,
    pub velocity: Vector2,

    pub aabb: AABB,
    pub aabb_offset: Vector2,

    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,

    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,

    pub was_on_ground: bool,
    pub on_ground: bool,

    pub was_at_ceiling: bool,
    pub at_ceiling: bool,
}

impl MovingObject {
    pub fn new(position: Vector2, size: Vector2) -> MovingObject {
        let aabb = AABB::new_full(position.clone(), size.clone());


        MovingObject {
            old_position: position.clone(),
            position: position.clone(),
            old_velocity: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            aabb_offset: Vector2::new(0.0, 0.0),
            aabb: aabb,
            pushed_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            pushes_right_wall: false,
            was_on_ground: false,
            on_ground: false,
            was_at_ceiling: false,
            at_ceiling: false,
        }
    }

    pub fn update_physics(&mut self, time: &Duration, terrain: &Terrain) {
        self.old_position = self.position;
        self.old_velocity = self.velocity;

        self.was_on_ground = self.on_ground;
        self.was_at_ceiling = self.at_ceiling;
        self.pushed_left_wall = self.pushes_left_wall;
        self.pushed_right_wall = self.pushes_right_wall;

        self.position += self.velocity * seconds(time);

        let mut ground_y = 0.0;

        if self.velocity.y <= 0.0 && self.has_ground(&mut ground_y, terrain) {
            self.position.y = ground_y + self.aabb.half_size.y - self.aabb_offset.y;
            println!("Ground here: {}", self.position);
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }

        self.aabb.center = self.position + self.aabb_offset;
    }

    pub fn has_ground(&self, ground_y: &mut f64, terrain: &Terrain) -> bool {

        let center = self.position + self.aabb_offset;
        let bottom_left = center - self.aabb.half_size + Vector2::new(1.0, -1.0);
        let bottom_right = Vector2::new(
            bottom_left.x + self.aabb.half_size.x * 2.0 - 2.0,
            bottom_left.y,
        );

        let mut tile_index_x;
        let mut tile_index_y;

        let mut checked_tile = bottom_left;

        loop {
            checked_tile.x = checked_tile.x.min(bottom_right.x);

            tile_index_x = terrain.get_tile_x_at_point(checked_tile.x);
            tile_index_y = terrain.get_tile_y_at_point(checked_tile.y);

            *ground_y = tile_index_y as f64 * terrain.tile_size + terrain.tile_size / 2.0 +
                terrain.position.y;

            if terrain.is_obstacle(tile_index_x, tile_index_y) {
                println!("Tile below: ({}, {})", tile_index_x, tile_index_y);
                return true;
            }

            if checked_tile.x >= bottom_right.x {
                break;
            }

            checked_tile.x += terrain.tile_size;
        }

        false
    }
}


pub fn seconds(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + (dur.subsec_nanos() as f64 / 1000000000.0)
}
