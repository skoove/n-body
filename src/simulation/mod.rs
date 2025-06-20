use bevy::prelude::*;

use crate::particle::{despawn_particles, Particle};

pub mod collisions;
pub mod gravity;
pub mod motion;
pub mod quadtree;

const PHYSICS_UPDATE_HZ: f64 = 120.0;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                // quadtree::quadtree_system,
                motion::update_particle_positions,
                gravity::calc_grav_accel,
                collisions::calculate_collisions,
            )
                .chain()
                .run_if(sim_not_paused),
        )
        .add_systems(
            Update,
            clear_particles_system.run_if(should_clear_particles),
        )
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_UPDATE_HZ))
        .init_resource::<SimSettings>()
        .init_resource::<quadtree::QuadTree>();
    }
}

/// returns true if the simulation is not paused
pub fn sim_not_paused(settings: Res<SimSettings>) -> bool {
    !settings.paused
}

fn clear_particles_system(
    particles: Query<Entity, With<Particle>>,
    mut commands: Commands,
    mut settings: ResMut<SimSettings>,
) {
    despawn_particles(&mut commands, particles);
    settings.should_clear_all_particles = false;
}

fn should_clear_particles(settings: Res<SimSettings>) -> bool {
    settings.should_clear_all_particles
}

#[derive(Resource)]
pub struct SimSettings {
    pub paused: bool,
    pub gravity_constant: f32,
    pub collision_steps: u32,
    pub enable_collisions: bool,
    pub should_clear_all_particles: bool,
}

impl Default for SimSettings {
    fn default() -> Self {
        SimSettings {
            paused: true,
            gravity_constant: 500.0,
            collision_steps: 2,
            enable_collisions: false,
            should_clear_all_particles: false,
        }
    }
}

impl SimSettings {
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        info!("toggle pause")
    }
}
