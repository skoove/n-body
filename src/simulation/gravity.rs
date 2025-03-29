use crate::particle::Mass;
use crate::simulation::motion::Acceleration;
use crate::simulation::SimSettings;
use bevy::prelude::*;

pub fn calc_grav_accel(
    mut query: Query<(Entity, &mut Acceleration, &Mass, &Transform)>,
    time: Res<Time>,
    sim_settings: Res<SimSettings>,
) {
    let delta_time = time.delta_secs();

    for (_, mut accel, _, _) in query.iter_mut() {
        accel.0 = Vec2::ZERO;
    }

    let entities: Vec<(Entity, f32, Vec3)> = query
        .iter()
        .map(|(entity, _accel, mass, transform)| (entity, mass.0, transform.translation))
        .collect();

    for (entity, mut accel, _mass, transform) in query.iter_mut() {
        for (other_entity, other_mass, other_translation) in &entities {
            if entity == *other_entity {
                continue;
            }
            let delta = transform.translation - *other_translation;
            let d_sq = delta.length_squared();
            let d = d_sq.sqrt();
            let direction = delta / d;
            accel.0 += sim_settings.gravity_constant
                * (-other_mass * direction.truncate() / d_sq)
                * delta_time;
        }
    }
}
