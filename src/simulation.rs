use crate::*;

pub struct MotionPlugin;

const G: f32 = 150000.0;

#[derive(Component)]
pub struct PreviousPosition(pub Transform);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (calc_grav_accel, update_particle_positions).chain());
    }
}

fn update_particle_positions(mut query: Query<&mut Transform, With<Particle>>, time: Res<Time>) {
    todo!()
}

fn calc_grav_accel(
    mut query: Query<(Entity, &mut Acceleration, &Mass, &Transform)>,
    time: Res<Time>,
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
            let d = d_sq.sqrt().max(50.0);
            let direction = delta / d;
            accel.0 += G * (-other_mass * direction.truncate() / d_sq) * delta_time;
        }
    }
}
