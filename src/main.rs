extern crate ggez;
extern crate marker;
extern crate nalgebra as na;
extern crate rand;
extern crate serde_json;


pub type Point2 = na::Point2<f64>;
pub type Vector2 = na::Vector2<f64>;

mod sprite;
mod state;
mod player;
mod level;
mod camera;
mod physics;
mod debug;

use ggez::conf;
use ggez::event;
use ggez::timer;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use std::rc::Rc;
use std::time::Duration;

use debug::Debug;
use state::StateMachine;
use player::*;
use camera::*;

pub struct Game {
    pub player: Player,
    pub player_sm: StateMachine,
    pub level: Rc<RenderableLevel>,
    pub camera: Camera,
    pub fixed_update: Duration,
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

        Ok(Game {
            player: p,
            player_sm: sm,
            level,
            camera: Camera::new(w, h, 1600.0, 1000.0),
            fixed_update: Duration::from_secs(0),
        })
    }
}


impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        // let update_start = timer::get_time_since_start(ctx);

        self.player_sm.handle_events(&mut self.player);

        self.player_sm.update(
            &mut self.player,
            &dt,
            &self.level.terrain,
        );
        if timer::check_update_time(ctx, 30) {
            self.player_sm.fixed_update(&mut self.player);
            self.fixed_update = Duration::from_secs(0);
        } else {
            self.fixed_update += dt;
        };
        self.camera.move_to(self.player.mv.position);

        // let update_end = timer::get_time_since_start(ctx);
        // let delta = update_end - update_start;
        // println!("Update: {}", physics::seconds(&delta));
        println!("Fps: {}", timer::get_fps(ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let camera = &self.camera;

        self.level.level.assets.background.draw_camera(
            camera,
            ctx,
            graphics::Point::new(
                camera.location().x as f32,
                camera.location().y as f32,
            ),
            0.0,
        )?;

        self.player_sm.draw(ctx, camera, &self.player);

        Debug::draw_level_obstacles(ctx, &*self.level, camera);

        for &(ref img, ref dp) in self.level.sprites.iter() {
            (&**img).draw_ex_camera(camera, ctx, dp.clone())?;
        }

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
}

use level::*;

pub fn main() {
    let c = conf::Conf {
        window_height: 1000,
        window_width: 1600,
        resizable: false,
        vsync: false,
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    let mut state = Game::new(ctx).unwrap();


    event::run(ctx, &mut state).unwrap();
}
