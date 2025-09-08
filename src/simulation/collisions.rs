use crate::{
    particle::{Particle, Radius},
    simulation::motion::OldPosition,
};
use bevy::prelude::*;

use super::SimSettings;

pub fn calculate_collisions(
    mut particles: Query<(&mut Transform, &OldPosition, &Radius), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    if !sim_settings.enable_collisions {
        return;
    }
    for _ in 0..sim_settings.collision_steps {
        let mut iter = particles.iter_combinations_mut();
        while let Some(
            [(mut pos1, old_pos1, Radius(radius1)), (mut pos2, old_pos2, Radius(radius2))],
        ) = iter.fetch_next()
        {
            let distance = pos1.translation - pos2.translation;
            let distance_length = distance.length();

            if distance_length < radius1 + radius2 {
                let overlap = radius1 + radius2 - distance_length;

                let velocity_1 = (pos1.translation - old_pos1.0.translation).normalize();
                let velocity_2 = (pos2.translation - old_pos2.0.translation).normalize();

                let velocity_vector = velocity_1.dot(velocity_2);

                let move_distance = overlap / 2.0;
                pos1.translation += velocity_vector * move_distance;
                pos2.translation -= velocity_vector * move_distance;
            }
        }
    }
}
