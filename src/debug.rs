use ggez::Context;
use super::camera::*;
use super::level::*;
use super::player::Player;
use ggez::graphics;


pub struct Debug;

impl Debug {
    pub fn draw_level_obstacles(ctx: &mut Context, level: &RenderableLevel, camera: &Camera) {
        if false {
            //Debug::DEBUG {
            let (ref img, ref dp) = level.sprites[0];
            let mut dp = dp.clone();
            let t = &level.terrain;

            for y in 0..t.height {
                for x in 0..t.width {
                    if t.get_tile(x as isize, y as isize) == TileType::Block {
                        let pos = t.get_map_tile_position(x as isize, y as isize);
                        let dx = pos.x as f32;
                        let dy = pos.y as f32;

                        dp.dest.x = dx;
                        dp.dest.y = dy;

                        (&**img).draw_ex_camera(camera, ctx, dp.clone()).unwrap();
                    }
                }
            }
        };
    }

    pub fn draw_aabb(ctx: &mut Context, player: &Player, camera: &Camera) {
        if Debug::DEBUG {
            let dd = camera.calculate_dest_point(player.mv.position + player.mv.aabb.offset);
            let rect = graphics::Rect::new(
                dd.x,
                dd.y,
                player.mv.aabb.half_size().x as f32 * 2.0,
                player.mv.aabb.half_size().y as f32 * 2.0,
            );
            graphics::set_color(ctx, graphics::WHITE).unwrap();
            graphics::rectangle(ctx, graphics::DrawMode::Line, rect).unwrap();
        };
    }

    const DEBUG: bool = true;
}
