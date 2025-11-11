
use bevy_ecs::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode { Edit, Play }

pub struct EngineApp {
    pub world: World,
    pub edit_schedule: Schedule,
    pub play_schedule: Schedule,
    pub mode: Mode,
}

impl Default for EngineApp {
    fn default() -> Self {
        Self {
            world: World::new(),
            edit_schedule: Schedule::default(),
            play_schedule: Schedule::default(),
            mode: Mode::Edit,
        }
    }
}

impl EngineApp {
    pub fn update(&mut self) {
        match self.mode {
            Mode::Edit => self.edit_schedule.run(&mut self.world),
            Mode::Play => self.play_schedule.run(&mut self.world),
        }
    }
}

pub fn set_mode(app: &mut EngineApp, mode: Mode) { app.mode = mode; }
