use bevy::prelude::*;

#[derive(Component)]
pub struct OldPosition(pub Transform);

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
            let velocity = position.translation - old_position.0.translation;

            old_position.0.translation = position.translation;

            position.translation =
                (position.translation.truncate() + velocity.truncate() + acceleration.0 * dt * dt)
                    .extend(0.0);
            previous_acceleration.0 = acceleration.0;
            acceleration.0 = Vec2::ZERO;
        },
    );
}
