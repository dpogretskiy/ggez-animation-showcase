extern crate ggez;
extern crate marker;
extern crate serde_json;
extern crate rand;

extern crate nalgebra as na;
pub type Point2 = na::Point2<f64>;
pub type Vector2 = na::Vector2<f64>;

mod sprite;
mod state;
mod player;
mod level;
mod camera;

use ggez::conf;
use ggez::event;
use ggez::timer;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};


use std::time::Duration;

use state::StateMachine;
use player::Player;
use camera::*;

pub struct Game {
    pub player: Player,
    pub player_sm: StateMachine,
    pub level: RenderableLevel,
    pub camera: Camera,
}


impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let (p, sm) = Player::new(ctx)?;
        let level = {
            let l = Level::load(ctx, LevelType::Graveyard).unwrap();
            let rl = RenderableLevel::build(l);
            rl
        };

        let (w, h) = (ctx.conf.window_width, ctx.conf.window_height);

        Ok(Game {
            player: p,
            player_sm: sm,
            level,
            camera: Camera::new(w, h, 1600.0, 1000.0)
        })
    }
}


impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.player_sm.handle_events(&mut self.player);
        self.player_sm.update(&mut self.player);

        //ayy
        self.player.coordinates += self.player.velocity;

        self.camera.move_to(self.player.coordinates);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let camera = &self.camera;

        self.player_sm.draw(ctx, camera, &self.player);

        for &(ref img, ref dp, ref attr) in self.level.sprites.iter() {
            (&**img).draw_ex_camera(camera, ctx, dp.clone())?;
        }

        graphics::present(ctx);

        timer::sleep_until_next_frame(ctx, 30);
        self.player_sm.fixed_update(&mut self.player);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => self.player.player_input.left = true,
                Keycode::Right => self.player.player_input.right = true,
                Keycode::Up => self.player.player_input.up = true,
                Keycode::Down => self.player.player_input.down = true,
                Keycode::LCtrl => self.player.player_input.slide = true,
                Keycode::Space => self.player.player_input.jump = true,
                Keycode::LShift => self.player.player_input.attack = true,
                _ => (),
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            //wat?
            match keycode {
                Keycode::Left => self.player.player_input.left = false,
                Keycode::Right => self.player.player_input.right = false,
                Keycode::Up => self.player.player_input.up = false,
                Keycode::Down => self.player.player_input.down = false,
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
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    let mut state = Game::new(ctx).unwrap();


    event::run(ctx, &mut state).unwrap();
}
