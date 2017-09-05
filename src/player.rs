use state::*;

use ggez::{Context, GameResult};

use std::boxed::Box;
use std::time::Duration;

use ggez::graphics::*;
use ggez::graphics;

use super::camera::*;
use super::physics::MovingObject;
use super::physics::seconds;
use super::level::Terrain;
use sprite::Loader;
use sprite::animation::Animated;

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Player {
    data: PlayerData,
    pub input: PlayerInput,
    pub direction: Direction,
    pub mv: MovingObject,
}

impl Player {
    pub fn new(ctx: &mut Context) -> GameResult<(Player, StateMachine)> {
        let scale = 0.4;
        let data = PlayerData::new(ctx, scale)?;
        let mut p = Player {
            data,
            input: PlayerInput {
                left: false,
                right: false,
                down: false,
                up: false,
                slide: false,
                jump: false,
                attack: false,
            },
            direction: Direction::Right,
            mv: MovingObject::new(
                Vector2::new(300.0, 800.0),
                Vector2::new(290.0 * scale, 500.0 * scale),
            ),
        };

        let mut sm = StateMachine::new(Idle);
        sm.start(&mut p);

        Ok((p, sm))
    }

    pub fn direct(&mut self) {
        if self.input.left ^ self.input.right {
            if self.input.left {
                self.direction = Direction::Left;
            } else {
                self.direction = Direction::Right;
            }
        };
    }

    const GRAVITY: f64 = -3000.0;
    const MAX_FALLING_SPEED: f64 = -4000.0;
    const JUMP_SPEED: f64 = 1600.0;
    const WALK_SPEED: f64 = 700.0;
}

fn draw_animation_frame(
    player: &Player,
    ctx: &mut Context,
    camera: &Camera,
    ss: &Animated,
    direction: &Direction,
) -> GameResult<()> {
    let d: f32 = match *direction {
        Direction::Left => -player.data.scale,
        Direction::Right => player.data.scale,
    };

    let dest = Point::new(player.mv.position.x as f32, player.mv.position.y as f32);

    (&*ss.marked_tiles.image).draw_ex_camera(
        camera,
        ctx,
        DrawParam {
            src: ss.current_frame_rect(),
            dest,
            rotation: 0.0,
            scale: Point::new(d, player.data.scale),
            offset: Point::new(0.0, 0.0),
            ..Default::default()
        },
    )?;

    if super::debug {
        let mut rect = ss.current_frame_rect();
        let dd = camera.calculate_dest_point(player.mv.position.clone());
        rect.x = dd.x;
        rect.y = dd.y;
        rect.w = player.mv.aabb.half_size.x as f32 * 2.0;
        rect.h = player.mv.aabb.half_size.y as f32 * 2.0;
        graphics::set_color(ctx, WHITE)?;
        graphics::rectangle(ctx, DrawMode::Line, rect)?;
    };

    Ok(())
}
pub struct PlayerData {
    scale: f32,
    idle: Animated,
    running: Animated,
    jumping: Animated,
    attacking: Animated,
    sliding: Animated,
}

impl PlayerData {
    pub fn new(ctx: &mut Context, scale: f64) -> GameResult<PlayerData> {
        let idle = Loader::load_sprite_sheet(ctx, "/idle")?;
        let attacking = Loader::load_sprite_sheet(ctx, "/attack")?;
        let jumping = Loader::load_sprite_sheet(ctx, "/jump")?;
        let running = Loader::load_sprite_sheet(ctx, "/run")?;
        let sliding = Loader::load_sprite_sheet(ctx, "/slide")?;

        Ok(PlayerData {
            scale: scale as f32,
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

impl State for Idle {
    fn on_start(&mut self, player: &mut Player) {
        player.data.idle.reset();
        player.mv.velocity = Vector2::new(0.0, 0.0);
    }
    fn on_resume(&mut self, player: &mut Player) {
        self.on_start(player);
    }
    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self, player: &mut Player) -> Trans {
        player.direct();

        let pi = &mut player.input;
        let mv = &mut player.mv;

        let trans = if !mv.on_ground {
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = Player::JUMP_SPEED;
            Trans::Push(Box::new(Jumping))
        } else if pi.down {
            if mv.on_platform {
                mv.position.y -= MovingObject::PLATFORM_THRESHOLD;
            };
            Trans::Push(Box::new(Jumping))
        } else if pi.left ^ pi.right {
            Trans::Push(Box::new(Running))
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

    fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
        player.mv.update_physics(duration, terrain);
        Trans::None
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        player.data.idle.roll_frames();
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(player, ctx, camera, &player.data.idle, &player.direction).unwrap();
    }
}
pub struct Running;

impl State for Running {
    fn on_start(&mut self, player: &mut Player) {
        player.data.running.reset();
    }
    fn on_resume(&mut self, player: &mut Player) {
        self.on_start(player);
    }
    fn on_pause(&mut self, _player: &mut Player) {}
    fn on_stop(&mut self, _player: &mut Player) {}

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        player.direct();
        let pi = &mut player.input;
        let mv = &mut player.mv;

        if !(pi.left ^ pi.right) {
            return Trans::Switch(Box::new(Idle));
        };

        let t = if !mv.on_ground {
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = Player::JUMP_SPEED;
            Trans::Push(Box::new(Jumping))
        } else if pi.down {
            if mv.on_platform {
                mv.position.y -= MovingObject::PLATFORM_THRESHOLD;
            }
            Trans::Push(Box::new(Jumping))
        } else if pi.slide {
            Trans::Push(Box::new(Sliding))
        } else if pi.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };

        pi.reset_actions();
        t
    }

    fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
        match player.direction {
            Direction::Left => if player.mv.pushes_left_wall {
                player.mv.velocity.x = 0.0;
            } else {
                player.mv.velocity.x = -Player::WALK_SPEED;
            },
            Direction::Right => if player.mv.pushes_right_wall {
                player.mv.velocity.x = 0.0;
            } else {
                player.mv.velocity.x = Player::WALK_SPEED;
            },
        }

        player.mv.update_physics(duration, terrain);
        Trans::None
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        player.data.running.cycle_frames();
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(player, ctx, camera, &player.data.running, &player.direction).unwrap();
    }
}

pub struct Jumping;

impl State for Jumping {
    fn on_start(&mut self, player: &mut Player) {
        player.data.jumping.reset();
    }

    fn on_resume(&mut self, player: &mut Player) {
        self.on_start(player)
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        player.direct();

        if player.input.left ^ player.input.right {
            match player.direction {
                Direction::Left => if player.mv.pushes_left_wall {
                    player.mv.velocity.x = 0.0;
                } else {
                    player.mv.velocity.x = -Player::WALK_SPEED;
                },
                Direction::Right => if player.mv.pushes_right_wall {
                    player.mv.velocity.x = 0.0;
                } else {
                    player.mv.velocity.x = Player::WALK_SPEED;
                },
            };
        } else {
            player.mv.velocity.x = 0.0;
        };

        let t = if player.input.attack {
            Trans::Switch(Box::new(Attacking))
        } else {
            Trans::None
        };

        player.input.reset_actions();
        t
    }

    fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
        let y_vel = Player::GRAVITY * seconds(&duration) + player.mv.velocity.y;
        player.mv.velocity.y = y_vel.max(Player::MAX_FALLING_SPEED);
        player.mv.update_physics(duration, terrain);

        if player.mv.on_ground {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        player.data.jumping.roll_frames();
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(player, ctx, camera, &player.data.jumping, &player.direction).unwrap();
    }
}

pub struct Sliding;

impl State for Sliding {
    fn on_start(&mut self, player: &mut Player) {
        player.data.sliding.reset();
    }

    fn handle_events(&mut self, player: &mut Player) -> Trans {
        let t = if player.input.jump {
            Trans::Switch(Box::new(Jumping))
        } else {
            Trans::None
        };

        player.input.reset_actions();
        t
    }

    fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
        player.mv.update_physics(duration, terrain);
        Trans::None
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
        draw_animation_frame(player, ctx, camera, &player.data.sliding, &player.direction).unwrap();
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
        player.direct();

        let t = if self.can_cancel(player) {
            if player.input.jump {
                Trans::Switch(Box::new(Jumping))
            } else if player.input.slide {
                Trans::Switch(Box::new(Sliding))
            } else if player.input.attack {
                Trans::Switch(Box::new(Attacking))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        };

        player.input.reset_actions();
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

    fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
        player.mv.update_physics(duration, terrain);
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
        draw_animation_frame(
            player,
            ctx,
            &camera,
            &player.data.attacking,
            &player.direction,
        ).unwrap();
    }
}
