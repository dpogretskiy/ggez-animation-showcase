
use ggez::{Context, GameResult};
use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::{event, graphics, timer};
use ggez::graphics::Drawable;

use std::rc::Rc;
use std::time::Duration;

use debug::Debug;
use state::StateMachine;

use physics;
use level::*;
use game_box::*;
use player::*;
use camera::*;

pub struct Game {
    pub player: Player,
    pub player_sm: StateMachine,
    pub level: Rc<RenderableLevel>,
    pub camera: Camera,
    pub fixed_update: Duration,
    pub boxes: Vec<GameBox>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let level = Rc::new({
            let l = Level::load(ctx, LevelType::Graveyard).unwrap();
            let rl = RenderableLevel::build(l);
            rl
        });
        let (p, sm) = Player::new(ctx)?;
        let (w, h) = (ctx.conf.window_width, ctx.conf.window_height);
        let hc = h as f64 / w as f64;
        let fov = w as f64 * 1.5;

        Ok(Game {
            player: p,
            player_sm: sm,
            level,
            camera: Camera::new(w, h, fov, hc * fov),
            fixed_update: Duration::from_secs(0),
            boxes: vec![],
        })
    }

    // pub fn update_areas(&mut self, time: &Duration) {
    //     let mut w = physics::world::World::new();
    // }
}


impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        // let update_start = timer::get_time_since_start(ctx);

        let mut w = physics::world::World::new();
        w.update_areas(&self.camera);

        self.player_sm.handle_events(&mut self.player);

        self.player_sm
            .update(&mut self.player, &dt, &self.level.terrain);
        if timer::check_update_time(ctx, 30) {
            self.player_sm.fixed_update(&mut self.player);
        };

        for mut b in self.boxes.iter_mut() {
            b.update(&dt, &self.level.terrain)
        }


        self.camera.move_to(self.player.mv.position);
        // let update_end = timer::get_time_since_start(ctx);
        // let delta = update_end - update_start;
        // println!("Fps: {}", timer::get_fps(ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let camera = &self.camera;

        let bd_dp = graphics::DrawParam {
            src: graphics::Rect::new(0.0, 0.0, 1.0, 1.0),
            scale: graphics::Point::new(2.0, 2.0),
            dest: graphics::Point::new(
                camera.location().x as f32 * 0.9,
                camera.location().y as f32 * 0.9,
            ),
            ..Default::default()
        };

        self.level.background.draw_ex_camera(camera, ctx, bd_dp)?;

        self.player_sm.draw(ctx, camera, &self.player);

        // for b in self.boxes.iter() {
        //     b.draw_cam(ctx, camera);
        // }

        for batch in self.level.sprites.iter() {
            batch.draw_ex_camera(camera, ctx, graphics::DrawParam::default())?;
        }

        Debug::draw_level_obstacles(ctx, &self.level.terrain, camera);
        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => self.player.input.left = true,
                Keycode::Right => self.player.input.right = true,
                Keycode::Up => self.player.input.up = true,
                Keycode::Down => self.player.input.down = true,
                Keycode::LCtrl => self.player.input.slide = true,
                Keycode::Space => self.player.input.jump = true,
                Keycode::LShift => self.player.input.attack = true,
                _ => (),
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            //wat?
            match keycode {
                Keycode::Left => self.player.input.left = false,
                Keycode::Right => self.player.input.right = false,
                Keycode::Up => self.player.input.up = false,
                Keycode::Down => self.player.input.down = false,
                _ => (),
            }
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32) {
        match btn {
            Button::A => self.player.input.jump = true,
            Button::X => self.player.input.attack = true,
            Button::B => self.player.input.slide = true,
            Button::LeftShoulder => self.player.mv.position = Vector2::new(300.0, 500.0),
            _ => (),
        }

        Debug::gamepad_button(btn, _instance_id);
    }
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, axis: Axis, value: i16, _instance_id: i32) {
        match axis {
            Axis::LeftX => {
                if value > 7500 {
                    self.player.input.right = true
                } else {
                    self.player.input.right = false
                };
                if value < -7500 {
                    self.player.input.left = true
                } else {
                    self.player.input.left = false
                }
            }
            Axis::LeftY => if value > 7500 {
                self.player.input.down = true
            } else {
                self.player.input.down = false
            },
            _ => (),
        }

        Debug::gamepad_axis(axis, value);
    }

    fn mouse_button_down_event(&mut self, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left {
            let p = self.camera.screen_to_world_coords((x, y));
            let rect = graphics::Rect::new(0.0, 0.41133004, 0.25728154, 0.26108375);
            // let rect = graphics::Rect::new(0.0, 0.0, 1.0, 1.0);

            // self.boxes.push(GameBox::new(
            //     p,
            //     &self.level.objects.image,
            //     rect,
            // ));
        }
    }
}
