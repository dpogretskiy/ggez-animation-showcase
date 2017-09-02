use ggez::Context;
use ggez::graphics::Point;

use super::player::*;
use super::camera::*;

use std::fmt::{Debug, Formatter};
use std::result::Result;
use std::fmt::Error;

#[derive(Debug)]
pub enum Trans {
    None,
    Pop,
    Push(Box<State>),
    Switch(Box<State>),
    Quit,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("state!");
        Ok(())
    }
}

pub trait State {
    fn on_start(&mut self, player: &mut Player) {}
    fn on_stop(&mut self, player: &mut Player) {}
    fn on_pause(&mut self, player: &mut Player) {}
    fn on_resume(&mut self, player: &mut Player) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self, player: &mut Player) -> Trans {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(&mut self, player: &mut Player) -> Trans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self, player: &mut Player) -> Trans {
        Trans::None
    }

    fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {}
}

#[derive(Debug)]
pub struct StateMachine {
    running: bool,
    state_stack: Vec<Box<State>>,
}

impl StateMachine {
    pub fn new<T>(initial_state: T) -> StateMachine
    where
        T: State + 'static,
    {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, camera: &Camera, player: &Player) {
        if let Some(state) = self.state_stack.last_mut() {
            state.draw(ctx, player, camera);
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self, player: &mut Player) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(player);
            self.running = true;
        }
    }

    pub fn handle_events(&mut self, player: &mut Player) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_events(player),
                None => Trans::None,
            };

            self.transition(trans, player);
        }
    }

    pub fn fixed_update(&mut self, player: &mut Player) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.fixed_update(player),
                None => Trans::None,
            };

            self.transition(trans, player);
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(player),
                None => Trans::None,
            };

            self.transition(trans, player);
        }
    }

    fn transition(&mut self, request: Trans, player: &mut Player) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(player),
                Trans::Push(state) => self.push(state, player),
                Trans::Switch(state) => self.switch(state, player),
                Trans::Quit => self.stop(player),
            }
        }
    }

    fn switch(&mut self, state: Box<State>, player: &mut Player) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(player)
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(player);
        }
    }

    fn push(&mut self, state: Box<State>, player: &mut Player) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(player);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(player);
        }
    }

    fn pop(&mut self, player: &mut Player) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(player);
            }

            if let Some(mut state) = self.state_stack.last_mut() {
                state.on_resume(player);
            } else {
                self.running = false;
            }
        }
    }

    fn stop(&mut self, player: &mut Player) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(player);
            }

            self.running = false;
        }
    }
}
