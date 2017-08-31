extern crate ggez;
extern crate marker;
extern crate serde_json;
extern crate rand;

mod sprite;
mod state;
mod player;
mod level;

use ggez::conf;
use ggez::event;
use ggez::timer;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::graphics::{Point, Rect};
use ggez::event::{Keycode, Mod};


use std::time::Duration;

use state::StateMachine;
use player::Player;

pub struct Game {
    pub player: Player,
    pub player_sm: StateMachine,
    pub level: RenderableLevel,
}


impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let (p, sm) = Player::new(ctx)?;
        let level = {
            let l = Level::load(ctx, LevelType::Graveyard).unwrap();
            let rl = RenderableLevel::build(l);
            rl
        };

        Ok(Game {
            player: p,
            player_sm: sm,
            level,
        })
    }
}


impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.player_sm.update(&mut self.player);
        self.player_sm.handle_events(&mut self.player);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest = Point::new(800.0, 500.0);
        self.player_sm.draw(ctx, dest, &self.player);

        for &(ref img, ref dp, ref attr) in self.level.sprites.iter() {
            graphics::draw_ex(ctx, &**img, dp.clone())?;

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
use marker::*;

pub fn main() {
    let c = conf::Conf {
        window_height: 1000,
        window_width: 1600,
        resizable: true,
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    let mut state = Game::new(ctx).unwrap();


    event::run(ctx, &mut state).unwrap();
}
