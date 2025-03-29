use bevy::prelude::*;
use motion::MotionPlugin;
pub mod gravity;
pub mod motion;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MotionPlugin).init_resource::<SimSettings>();
    }
}

#[derive(Resource)]
pub struct SimSettings {
    pub paused: bool,
    pub gravity_constant: f32,
}

impl Default for SimSettings {
    fn default() -> Self {
        SimSettings {
            paused: true,
            gravity_constant: 15000.0,
        }
    }
}

impl SimSettings {
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        info!("toggle pause")
    }
}
