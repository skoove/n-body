use crate::particle::{Particle, Radius};
use bevy::prelude::*;

use super::SimSettings;

pub fn calculate_collisions(
    mut particles: Query<(&mut Transform, &Radius), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    if !sim_settings.enable_collisions {
        return;
    }
    for _ in 0..sim_settings.collision_substeps {
        let mut iter = particles.iter_combinations_mut();
        while let Some([(mut pos1, Radius(radius1)), (mut pos2, Radius(radius2))]) =
            iter.fetch_next()
        {
            let distance = pos1.translation - pos2.translation;
            let distance_length = distance.length();

            if distance_length < radius1 + radius2 {
                let overlap = radius1 + radius2 - distance_length;
                let collision_normal = distance.normalize();
                let move_distance = overlap / 2.0;
                pos1.translation += collision_normal * move_distance;
                pos2.translation -= collision_normal * move_distance;
            }
        }
    }
}
