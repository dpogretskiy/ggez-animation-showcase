use ggez::Context;
use ggez::event::*;
use ggez::graphics;
use ggez::graphics::DrawMode;

use super::camera::*;
use super::level::*;
use super::player::*;
use super::physics::*;

pub struct Debug;

impl Debug {
    const DEBUG: bool = false;

    pub fn draw_level_obstacles(ctx: &mut Context, terrain: &Terrain, camera: &Camera) {
        if Debug::DEBUG {
            for y in 0..terrain.height {
                for x in 0..terrain.width {
                    if terrain.get_tile(x as isize, y as isize) == TileType::Block {
                        let pos = terrain.get_map_tile_position(x as isize, y as isize);

                        let dest = camera.calculate_dest_point(pos);
                        let x = dest.x as f32;
                        let y = dest.y as f32;
                        let w = camera.draw_scale().x * terrain.tile_size as f32;
                        let h = camera.draw_scale().y * terrain.tile_size as f32;

                        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.03))
                            .unwrap();
                        graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(x, y, w, h))
                            .unwrap();
                        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.7)).unwrap();
                        graphics::rectangle(ctx, DrawMode::Line, graphics::Rect::new(x, y, w, h))
                            .unwrap();
                        graphics::set_color(ctx, graphics::WHITE).unwrap();
                    }
                }
            }
        };
    }

    pub fn draw_aabb(ctx: &mut Context, player: &Player, camera: &Camera) {
        if true {
            // if Debug::DEBUG {
            let dd = camera.calculate_dest_point(player.mv.position + player.mv.aabb.offset);
            let scale = camera.draw_scale();
            let rect = graphics::Rect::new(
                dd.x,
                dd.y,
                player.mv.aabb.half_size.x as f32 * 2.0 * scale.x,
                player.mv.aabb.half_size.y as f32 * 2.0 * scale.y,
            );

            let position = &player.mv.position;

            let bottom = {
                let s = player
                    .mv
                    .aabb
                    .sensor(position, Sensor::BottomLeft)
                    .right()
                    .down()
                    .ok();
                vec![
                    Vector2::new(s.x + player.mv.aabb.half_size.x * 2.0, s.y)
                        .left()
                        .left(),
                    s,
                ]
            };

            let top = {
                let s = player
                    .mv
                    .aabb
                    .sensor(position, Sensor::TopRight)
                    .left()
                    .up()
                    .ok();
                vec![
                    Vector2::new(s.x - player.mv.aabb.half_size.x * 2.0, s.y)
                        .right()
                        .right(),
                    s,
                ]
            };

            let sensors = vec![
                top,
                bottom,
                vec![
                    player.mv.aabb.sensor(position, Sensor::TopLeft).left().ok(),
                    player
                        .mv
                        .aabb
                        .sensor(position, Sensor::BottomLeft)
                        .left()
                        .ok(),
                ],
                vec![
                    player
                        .mv
                        .aabb
                        .sensor(position, Sensor::TopRight)
                        .right()
                        .ok(),
                    player
                        .mv
                        .aabb
                        .sensor(position, Sensor::BottomRight)
                        .right()
                        .ok(),
                ],
            ];

            graphics::set_color(ctx, graphics::Color::new(0.3, 0.3, 1.0, 1.0)).unwrap();
            graphics::rectangle(ctx, graphics::DrawMode::Line, rect).unwrap();

            graphics::set_color(ctx, graphics::Color::new(1.0, 0.3, 0.3, 1.0)).unwrap();

            for x in sensors.iter() {
                let points: Vec<graphics::Point> = x.iter()
                    .map(|v| camera.calculate_dest_point(v.clone()))
                    .collect();

                graphics::line(ctx, points.as_slice()).unwrap();
            }
            graphics::set_color(ctx, graphics::WHITE).unwrap();
        };
    }

    pub fn detect_teleporting(mv: &MovingObject) {
        if Debug::DEBUG {
            if (mv.old_position.x - mv.position.x).abs() > 30.0 {
                println!("X TELEPORTING: {:?}", mv);
            }
            if (mv.old_position.y - mv.position.y).abs() > 30.0 {
                println!("Y TELEPORTING: {:?}", mv);
            }
        }
    }

    pub fn gamepad_axis(axis: Axis, value: i16) {
        if Debug::DEBUG {
            println!("{:?}: {}", axis, value);
        }
    }

    pub fn gamepad_button(btn: Button, instance: i32) {
        if Debug::DEBUG {
            println!("INSTANCE: {}; Button: {:?}", instance, btn);
        }
    }
}
