use crate::gui::performance_gui::PerformanceData;
use bevy::prelude::*;
use motion::MotionPlugin;
pub mod collisions;
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
    pub collision_substeps: i32,
    pub enable_collisions: bool,
}

impl Default for SimSettings {
    fn default() -> Self {
        SimSettings {
            paused: true,
            gravity_constant: 500.0,
            collision_substeps: 2,
            enable_collisions: false,
        }
    }
}

impl SimSettings {
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        info!("toggle pause")
    }
}

fn collect_perfromance_data(mut perf_data: ResMut<PerformanceData>, time: Res<Time<Virtual>>) {
    perf_data
        .simulation_time
        .push_back(time.delta_secs() * 1000.0);
}
