use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "donttilethanks".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(
            Startup,
            (
                setup_camera,
                (spawn_big_particle, spawn_random_particles, show_particles).chain(),
            ),
        )
        .add_systems(
            Update,
            (
                calc_grav_accel,
                update_particle_velocities,
                update_particle_positions,
            )
                .chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn show_particles(
    mut commands: Commands,
    query: Query<(Entity, &Particle)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2d(meshes.add(Circle::new(1.0)));
    let material = MeshMaterial2d(materials.add(Color::hsv(0.0, 0.0, 1.0)));
    for (entity, _) in &query {
        commands
            .entity(entity)
            .insert((shape.clone(), material.clone()));
    }
}

fn update_particle_positions(
    mut query: Query<(&mut Transform, &Velocity), With<Particle>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    query.par_iter_mut().for_each(|(mut position, velocity)| {
        position.translation[0] += velocity.0 * delta_time;
        position.translation[1] += velocity.1 * delta_time;
    })
}

fn update_particle_velocities(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    let delta_time = time.delta_secs();
    query
        .par_iter_mut()
        .for_each(|(mut velocity, acceleration)| {
            velocity.0 += acceleration.0 * delta_time;
            velocity.1 += acceleration.1 * delta_time;
        })
}

fn calc_grav_accel(
    mut query: Query<(Entity, &mut Acceleration, &Mass, &Transform)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (_, mut accel, _, _) in query.iter_mut() {
        accel.0 = 0.0;
        accel.1 = 0.0;
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
            let d = d_sq.sqrt().max(150.0);
            let direction = delta / d;
            accel.0 += -other_mass * direction.x / d_sq * delta_time;
            accel.1 += -other_mass * direction.y / d_sq * delta_time;
        }
    }
}

fn spawn_random_particles(mut commands: Commands) {
    let amount_to_spawn = 2000;
    let spawn_radius: f32 = 400.0;
    let velocity_range = -100.0..100.0;
    let mut rng = rand::rng();
    for _ in 0..amount_to_spawn {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        let radius: f32 = rng.random_range(0.0..spawn_radius).min(100.0);

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let x_v = rng.random_range(velocity_range.clone());
        let y_v = rng.random_range(velocity_range.clone());
        commands.spawn((
            Particle,
            Transform::from_xyz(x, y, 0.0),
            Mass(30000.0),
            Velocity(x_v, y_v),
            Acceleration(0.0, 0.0),
        ));
    }
}

fn spawn_big_particle(mut commands: Commands) {
    commands.spawn((
        Particle,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mass(5000000.0),
        Velocity(0.0, 0.0),
        Acceleration(0.0, 0.0),
    ));
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Acceleration(f32, f32);

#[derive(Component)]
struct Mass(f32);
