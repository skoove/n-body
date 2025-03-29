use crate::*;

pub struct MotionPlugin;

const G: f32 = 150000.0;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (calc_grav_accel, update_particle_positions)
                .chain()
                .run_if(sim_not_paused),
        )
        .init_resource::<SimSettings>();
    }
}

#[derive(Component)]
pub struct OldPosition(pub Transform);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

#[derive(Resource)]
pub struct SimSettings {
    paused: bool,
}

impl Default for SimSettings {
    fn default() -> Self {
        SimSettings { paused: true }
    }
}

/// returns true if the simulation is not paused
fn sim_not_paused(settings: Res<SimSettings>) -> bool {
    !settings.paused
}

fn update_particle_positions(
    mut query: Query<(&mut Transform, &mut OldPosition, &Acceleration), With<Particle>>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut position, mut old_position, acceleration)| {
            let dt = time.delta_secs();
            let velocity = position.translation - old_position.0.translation;
            old_position.0.translation = position.translation;

            position.translation =
                (position.translation.truncate() + velocity.truncate() + acceleration.0 * dt * dt)
                    .extend(0.0);
        });
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
