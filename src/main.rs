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
mod game;
mod game_box;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::Context;

use game::*;

pub fn main() {
    let c = conf::Conf {
        window_width: 1600,
        window_height: 1000,
        resizable: false,
        vsync: false,
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    let mut state = Game::new(ctx).unwrap();

    println!("{:?}", graphics::get_renderer_info(ctx));

    event::run(ctx, &mut state).unwrap();
}
