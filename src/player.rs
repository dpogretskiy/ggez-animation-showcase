use state::*;

use ggez::{Context, GameResult};

use std::boxed::Box;
use ggez::graphics::{DrawParam, Point};
use sprite::Loader;
use sprite::animation::Animated;
use super::camera::*;

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Player {
    data: PlayerData,
    pub player_input: PlayerInput,
    direction: Direction,
    pub coordinates: Vector2,
    pub velocity: Vector2,
}

impl Player {
    pub fn new(ctx: &mut Context) -> GameResult<(Player, StateMachine)> {
        let data = PlayerData::new(ctx)?;

        let mut p = Player {
            data,
            player_input: PlayerInput {
                left: false,
                right: false,
                down: false,
                up: false,
                slide: false,
                jump: false,
                attack: false,
            },
            direction: Direction::Right,
            coordinates: Vector2::new(300.0, 300.0),
            velocity: Vector2::new(0.0, 0.0),
        };

        let mut sm = StateMachine::new(Idle);
        sm.start(&mut p);

        Ok((p, sm))
    }
}

fn draw_animation_frame(
    player: &Player,
    ctx: &mut Context,
    camera: &Camera,
    ss: &Animated,
    direction: &Direction,
) -> GameResult<()> {
    let d: f32 = match *direction {
        Direction::Left => -0.4,
        Direction::Right => 0.4,
    };

    let dest = Point::new(player.coordinates.x as f32, player.coordinates.y as f32);

    (&*ss.marked_tiles.image).draw_ex_camera(
        camera,
        ctx,
        DrawParam {
            src: ss.current_frame_rect(),
            dest,
            rotation: 0.0,
            scale: Point::new(d, 0.4),
            offset: Point::new(0.0, 0.0),
            ..Default::default()
        },
    )?;

    Ok(())
}
pub struct PlayerData {
    idle: Animated,
    running: Animated,
    jumping: Animated,
    attacking: Animated,
    sliding: Animated,
}

impl PlayerData {
    pub fn new(ctx: &mut Context) -> GameResult<PlayerData> {
        let idle = Loader::load_sprite_sheet(ctx, "/idle")?;
        let attacking = Loader::load_sprite_sheet(ctx, "/attack")?;
        let jumping = Loader::load_sprite_sheet(ctx, "/jump")?;
        let running = Loader::load_sprite_sheet(ctx, "/run")?;
        let sliding = Loader::load_sprite_sheet(ctx, "/slide")?;

        Ok(PlayerData {
            idle: Animated::new(idle),
            jumping: Animated::new(jumping),
            running: Animated::new(running),
            attacking: Animated::new(attacking),
            sliding: Animated::new(sliding),
        })
    }
}

#[derive(Debug)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub slide: bool,
    pub jump: bool,
    pub attack: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
            slide: false,
            jump: false,
            attack: false,
        }
    }
}

impl PlayerInput {
    pub fn new() -> PlayerInput {
        PlayerInput::default()
    }

    pub fn reset_actions(&mut self) {
        self.attack = false;
        self.slide = false;
        self.jump = false;
    }
}

pub struct Idle;

fn direction(input: &PlayerInput, og: &Direction) -> Direction {
    if input.left == true {
        Direction::Left
    } else if input.right == true {
        Direction::Right
    } else {
        og.clone()
    }
}

impl State for Idle {
    fn on_start(&mut self, player: &mut Player) {
        player.data.idle.reset();
    }
    fn on_resume(&mut self, player: &mut Player) {
        player.data.idle.reset();
    }
    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let pi = &mut player.player_input;

        let dir = direction(&pi, &player.direction);
        player.direction = dir;

        if pi.left ^ pi.right {
            return Trans::Switch(Box::new(Running));
        };

        let trans = if pi.jump {
            Trans::Push(Box::new(Jumping))
        } else if pi.slide {
            Trans::Push(Box::new(Sliding))
        } else if pi.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };

        pi.reset_actions();
        trans
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        player.data.idle.cycle_frames();
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            camera,
            &player.data.idle,
            &player.direction,
        );
    }
}

pub struct Attacking;

impl Attacking {
    fn can_cancel(&self, player: &Player) -> bool {
        player.data.attacking.current_frame > 5
    }
}

impl State for Attacking {
    fn on_start(&mut self, player: &mut Player) {
        player.data.attacking.reset();
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let dir = direction(&player.player_input, &player.direction);
        player.direction = dir;

        let t = if self.can_cancel(player) {
            if player.player_input.jump {
                Trans::Switch(Box::new(Jumping))
            } else if player.player_input.slide {
                Trans::Switch(Box::new(Sliding))
            } else if player.player_input.attack {
                Trans::Switch(Box::new(Attacking))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        };

        player.player_input.reset_actions();
        t
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        if player.data.attacking.is_over() {
            Trans::Pop
        } else {
            player.data.attacking.next_frame();
            Trans::None
        }
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            &camera,
            &player.data.attacking,
            &player.direction,
        );
    }
}

pub struct Running;

impl State for Running {
    fn on_start(&mut self, player: &mut Player) {
        player.data.running.reset();
        let x = match player.direction {
            Direction::Left => -30.0,
            Direction::Right => 30.0,
        };
        let velo = Vector2::new(x, 0.0);
        player.velocity = velo;
    }
    fn on_resume(&mut self, player: &mut Player) {
        player.data.running.reset();
    }
    fn on_pause(&mut self, player: &mut Player) {
        player.velocity = Vector2::new(0.0, 0.0);
    }
    fn on_stop(&mut self, player: &mut Player) {
        self.on_pause(player);
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let dir = direction(&player.player_input, &player.direction);
        player.direction = dir;

        if !player.player_input.right && !player.player_input.left {
            return Trans::Switch(Box::new(Idle));
        };

        let t = if player.player_input.jump {
            Trans::Push(Box::new(Jumping))
        } else if player.player_input.slide {
            Trans::Push(Box::new(Sliding))
        } else if player.player_input.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };

        player.player_input.reset_actions();
        t
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        player.data.running.cycle_frames();
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            camera,
            &player.data.running,
            &player.direction,
        );
    }
}

pub struct Jumping;

impl State for Jumping {
    fn on_start(&mut self, player: &mut Player) {
        player.data.jumping.reset();
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let dir = direction(&player.player_input, &player.direction);
        player.direction = dir;

        let t = if player.player_input.attack {
            Trans::Switch(Box::new(Attacking))
        } else {
            Trans::None
        };

        player.player_input.reset_actions();
        t
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        if player.data.jumping.is_over() {
            Trans::Pop
        } else {
            player.data.jumping.next_frame();
            Trans::None
        }
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            camera,
            &player.data.jumping,
            &player.direction,
        );
    }
}

pub struct Sliding;

impl State for Sliding {
    fn on_start(&mut self, player: &mut Player) {
        player.data.sliding.reset();
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let t = if player.player_input.jump {
            Trans::Switch(Box::new(Jumping))
        } else {
            Trans::None
        };

        player.player_input.reset_actions();
        t
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        if player.data.sliding.is_over() {
            Trans::Pop
        } else {
            player.data.sliding.next_frame();
            Trans::None
        }
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            camera,
            &player.data.sliding,
            &player.direction,
        );
    }
}
