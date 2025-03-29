use crate::particle::{Mass, Particle};
use crate::simulation::motion::Acceleration;
use crate::simulation::SimSettings;
use bevy::prelude::*;

pub fn calc_grav_accel(
    mut query: Query<(&mut Acceleration, &Mass, &Transform), With<Particle>>,
    time: Res<Time>,
    sim_settings: Res<SimSettings>,
) {
    let dt = time.delta_secs();
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
        let distance = distance_sq.sqrt();
        let distance_cubed = distance * distance * distance;
        accel_1.0 +=
            (((sim_settings.gravity_constant * mass_2) / (distance_cubed)) * delta).truncate();
        accel_2.0 -=
            (((sim_settings.gravity_constant * mass_1) / (distance_cubed)) * delta).truncate();
    }
}
