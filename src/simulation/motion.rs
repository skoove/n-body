use super::collect_perfromance_data;
use super::collisions;
use super::gravity;
use crate::particle::Particle;
use crate::simulation::SimSettings;
use crate::PHYSICS_UPDATE_HZ;

use bevy::prelude::*;

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                gravity::calc_grav_accel,
                collisions::calculate_collisions,
                update_particle_positions,
                collect_perfromance_data,
            )
                .chain()
                .run_if(sim_not_paused),
        )
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_UPDATE_HZ));
    }
}

#[derive(Component)]
pub struct OldPosition(pub Transform);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

/// returns true if the simulation is not paused
fn sim_not_paused(settings: Res<SimSettings>) -> bool {
    !settings.paused
}

fn update_particle_positions(
    mut query: Query<(&mut Transform, &mut OldPosition, &mut Acceleration), With<Particle>>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut position, mut old_position, mut acceleration)| {
            let dt = time.delta_secs();
            let velocity = position.translation - old_position.0.translation;

            old_position.0.translation = position.translation;

            position.translation =
                (position.translation.truncate() + velocity.truncate() + acceleration.0 * dt * dt)
                    .extend(0.0);
            acceleration.0 = Vec2::ZERO;
        });
}
