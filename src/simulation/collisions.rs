use crate::{
    particle::{Mass, Particle, Radius},
    simulation::motion::OldPosition,
};
use bevy::prelude::*;

use super::SimSettings;

pub fn calculate_collisions(
    mut particles: Query<(&mut Transform, &Radius, &Mass), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    if !sim_settings.enable_collisions {
        return;
    }
    for _ in 0..sim_settings.collision_steps {
        let mut iter = particles.iter_combinations_mut();
        while let Some(
            [(mut position1, Radius(radius1), Mass(mass1)), (mut position2, Radius(radius2), Mass(mass2))],
        ) = iter.fetch_next()
        {
            let pos1 = position1.translation.xy();
            let pos2 = position2.translation.xy();

            let distance = pos1 - pos2;
            let distance_length = distance.length();

            if distance_length < radius1 + radius2 {
                let overlap = radius1 + radius2 - distance_length;
                let collision_normal = distance.normalize();
                let move_distance = overlap / 2.0;
                let correction = collision_normal * move_distance;

                position1.translation += correction.extend(0.0);
                position2.translation -= correction.extend(0.0);
            }
        }
    }
}
