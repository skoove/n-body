use crate::{gui::performance_gui::PerformanceData, PHYSICS_UPDATE_HZ};
use bevy::prelude::*;
pub mod collisions;
pub mod gravity;
pub mod motion;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                gravity::calc_grav_accel,
                collisions::calculate_collisions,
                motion::update_particle_positions,
            )
                .chain()
                .run_if(sim_not_paused),
        )
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_UPDATE_HZ))
        .init_resource::<SimSettings>();
    }
}

/// returns true if the simulation is not paused
fn sim_not_paused(settings: Res<SimSettings>) -> bool {
    !settings.paused
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
