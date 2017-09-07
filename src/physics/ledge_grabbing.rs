use super::*;
use level::Terrain;
use player::PlayerInput;

pub struct LedgeGrabbing {
    pub ledge_tile: (isize, isize),
}

impl LedgeGrabbing {
    const GRAB_LEDGE_START_Y: f64 = 0.0;
    const GRAB_LEDGE_END_Y: f64 = 2.0;
    const GRAB_LEDGE_TILE_OFFSET: f64 = -4.0;

    pub fn new() -> LedgeGrabbing {
        LedgeGrabbing { ledge_tile: (0, 0) }
    }

    pub fn grab_ledge(
        &mut self,
        mv: &mut MovingObject,
        pi: &PlayerInput,
        terrain: &Terrain,
    ) -> bool {
        if mv.velocity.y <= 0.0 && !mv.at_ceiling && (pi.right && mv.pushes_right_wall) ||
            (pi.left && mv.pushes_left_wall)
        {
            let aabb_corner_offset = if mv.pushes_right_wall {
                let mut hs = mv.aabb.half_size().clone();
                hs.x += 1.0;
                hs
            } else {
                Vector2::new(-mv.aabb.half_size().x - 1.0, mv.aabb.half_size().y)
            };

            let top_y;
            let bottom_y;

            let tile_x = terrain.get_tile_x_at_point(mv.aabb.center.x + aabb_corner_offset.x);

            if (mv.pushed_left_wall && mv.pushes_left_wall) ||
                (mv.pushed_right_wall && mv.pushes_right_wall)
            {
                top_y = terrain.get_tile_y_at_point(
                    mv.old_position.y + mv.aabb.offset.y + aabb_corner_offset.y -
                        LedgeGrabbing::GRAB_LEDGE_TILE_OFFSET,
                );
                bottom_y = terrain.get_tile_y_at_point(
                    mv.aabb.center.y + aabb_corner_offset.y -
                        LedgeGrabbing::GRAB_LEDGE_TILE_OFFSET,
                );
            } else {
                top_y = terrain.get_tile_y_at_point(
                    mv.aabb.center.y + aabb_corner_offset.y -
                        LedgeGrabbing::GRAB_LEDGE_TILE_OFFSET,
                );
                bottom_y = terrain.get_tile_y_at_point(
                    mv.aabb.center.y + aabb_corner_offset.y -
                        LedgeGrabbing::GRAB_LEDGE_END_Y,
                );
            };

            for y in (bottom_y..top_y + 1).rev() {
                if !terrain.is_obstacle(tile_x, y) && terrain.is_obstacle(tile_x, y - 1) {
                    let mut tile_corner = terrain.get_map_tile_position(tile_x, y - 1);

                    tile_corner.x -= aabb_corner_offset.x.signum() * terrain.tile_size / 2.0;
                    tile_corner.y += terrain.tile_size / 2.0;

                    if y > bottom_y ||
                        (mv.aabb.center.y + aabb_corner_offset.y) - tile_corner.y <=
                            LedgeGrabbing::GRAB_LEDGE_END_Y &&
                            tile_corner.y - (mv.aabb.center.y + aabb_corner_offset.y) >=
                                LedgeGrabbing::GRAB_LEDGE_START_Y
                    {
                        self.ledge_tile = (tile_x, y - 1);
                        mv.position.y = tile_corner.y - aabb_corner_offset.y - mv.aabb.offset.y -
                            LedgeGrabbing::GRAB_LEDGE_START_Y +
                            LedgeGrabbing::GRAB_LEDGE_TILE_OFFSET;

                        mv.velocity = Vector2::new(mv.velocity.x / 2.0, 0.0);
                        return true;
                    }
                };
            }
            return false;
        } else {
            false
        }
    }
}
