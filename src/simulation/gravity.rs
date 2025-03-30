use crate::particle::{Mass, Particle};
use crate::simulation::motion::Acceleration;
use crate::simulation::SimSettings;
use bevy::prelude::*;

pub fn calc_grav_accel(
    mut query: Query<(&mut Acceleration, &Mass, &Transform), With<Particle>>,
    sim_settings: Res<SimSettings>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(mut accel_1, Mass(mass_1), pos_1), (mut accel_2, Mass(mass_2), pos_2)]) =
        iter.fetch_next()
    {
        // a_a = (m_b/|r|^3) * r * dt * G
        let pos_1 = pos_1.translation;
        let pos_2 = pos_2.translation;
        let delta = pos_2 - pos_1;
        let distance_sq = delta.length_squared();
        if distance_sq < 1e-20 {
            continue;
        }
        let mut distance = distance_sq.sqrt();
        // if collisions are turned off, the particles get super close and start to be flung
        // away by the huge amount of force; this helps alot
        if !sim_settings.enable_collisions {
            distance = distance.max(25.0);
        }
        // even with collisions on things maybe will get flunng
        distance = distance.max(10.0);

        let distance_cubed = distance * distance * distance;
        accel_1.0 +=
            (((sim_settings.gravity_constant * mass_2) / (distance_cubed)) * delta).truncate();
        accel_2.0 -=
            (((sim_settings.gravity_constant * mass_1) / (distance_cubed)) * delta).truncate();
    }
}
