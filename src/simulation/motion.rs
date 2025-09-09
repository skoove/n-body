use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration(pub Vec2);

/// This componenet is intended to be used to render acceleration arrows
#[derive(Component)]
pub struct PreviousAcceleration(pub Vec2);

pub fn update_particle_positions(
    mut query: Query<(
        &mut Transform,
        &mut OldPosition,
        &mut Acceleration,
        &mut PreviousAcceleration,
    )>,
    time: Res<Time>,
) {
    query.par_iter_mut().for_each(
        |(mut position, mut old_position, mut acceleration, mut previous_acceleration)| {
            let dt = time.delta_secs();
            let pos = position.translation.truncate();
            let old_pos = old_position.0.translation.truncate();

            let new_pos = verlet_integrate(dt, pos, old_pos, acceleration.0);

            old_position.0 = *position;
            position.translation = new_pos.extend(0.0);

            previous_acceleration.0 = acceleration.0;
            acceleration.0 = Vec2::ZERO;
        },
    );
}

/// Returns the next position of an object from delta time, position, previous position and acceleration.
/// If using this to actually move things, remember to update old position and to reset acceleration at the end
pub fn verlet_integrate(
    delta_secs: f32,
    position: Vec2,
    old_position: Vec2,
    acceleration: Vec2,
) -> Vec2 {
    let velocity = position - old_position;
    position + velocity + acceleration * delta_secs * delta_secs
}
