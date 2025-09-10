use crate::{
    particle::{Mass, Particle, Radius},
    simulation::motion::OldPosition,
};
use bevy::prelude::*;

use super::SimSettings;

pub fn calculate_collisions(
    mut particles: Query<(&mut Transform, &mut OldPosition, &Radius, &Mass), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    if !sim_settings.enable_collisions {
        return;
    }
    for _ in 0..sim_settings.collision_steps {
        let mut iter = particles.iter_combinations_mut();
        while let Some(
            [(mut position1, mut old_position1, Radius(radius1), Mass(mass1)), (mut position2, mut old_position2, Radius(radius2), Mass(mass2))],
        ) = iter.fetch_next()
        {
            let pos1 = position1.translation.xy();
            let pos2 = position2.translation.xy();
            let old_pos1 = old_position1.0.translation.xy();
            let old_pos2 = old_position2.0.translation.xy();

            let distance = pos1 - pos2;
            let distance_length = distance.length();

            if distance_length < radius1 + radius2 {
                let overlap = radius1 + radius2 - distance_length;
                let collision_normal = distance.normalize();
                let move_distance = overlap / 2.0;
                let correction = collision_normal * move_distance;

                let vel1 = pos1 - old_pos1;
                let vel2 = pos2 - old_pos2;

                let new_vel1 =
                    calculate_collision_impulse(&pos1, &pos2, &vel1, &vel2, mass1, mass2);

                let new_vel2 =
                    calculate_collision_impulse(&pos2, &pos1, &vel2, &vel1, mass2, mass1);

                old_position1.0.translation = (pos1 - new_vel1).extend(0.0);
                old_position2.0.translation = (pos1 - new_vel2).extend(0.0);

                position1.translation += correction.extend(0.0);
                position2.translation -= correction.extend(0.0);
            }
        }
    }
}
// https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
fn calculate_collision_impulse(
    pos1: &Vec2,
    pos2: &Vec2,
    vel1: &Vec2,
    vel2: &Vec2,
    mass1: &f32,
    mass2: &f32,
) -> Vec2 {
    let rel_vel = vel1 - vel2;
    let radius = pos1 - pos2;
    let radius_len2 = radius.length_squared();
    vel1 - ((2.0 * mass2) / (mass1 + mass2)) * ((rel_vel.dot(radius)) / radius_len2) * radius
}
