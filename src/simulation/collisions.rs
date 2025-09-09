use crate::{
    particle::{Particle, Radius},
    simulation::motion::OldPosition,
};
use bevy::prelude::*;

use super::SimSettings;

pub fn calculate_collisions(
    mut particles: Query<(&mut Transform, &mut OldPosition, &Radius), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    if !sim_settings.enable_collisions {
        return;
    }
    for _ in 0..sim_settings.collision_steps {
        let mut iter = particles.iter_combinations_mut();
        while let Some(
            [(mut pos1, mut old_pos1, Radius(radius1)), (mut pos2, mut old_pos2, Radius(radius2))],
        ) = iter.fetch_next()
        {
            let distance = pos1.translation - pos2.translation;
            let distance_length = distance.length();

            if distance_length < radius1 + radius2 {
                let overlap = radius1 + radius2 - distance_length;
                let collision_normal = distance.normalize();
                let move_distance = overlap / 2.0;
                let correction = collision_normal * move_distance;

                let v1 = pos1.translation - old_pos1.0.translation;
                let v2 = pos2.translation - old_pos2.0.translation;
                let v1n = v1.dot(collision_normal);
                let v2n = v2.dot(collision_normal);

                let bouncyness = 0.90; // this is apparently called restitutian or something but this is more whimiscal
                let v1_new = v1 - (1.0 + bouncyness) * v1n * collision_normal;
                let v2_new = v2 - (1.0 + bouncyness) * v2n * collision_normal;

                old_pos1.0.translation = pos1.translation - v1_new;
                old_pos2.0.translation = pos2.translation - v2_new;

                pos1.translation += correction;
                pos2.translation -= correction;
            }
        }
    }
}
