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
                (spawn_random_particles, show_particles).chain(),
            ),
        )
        .add_systems(Update, update_particle_positions)
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
    let shape = Mesh2d(meshes.add(Circle::new(2.0)));
    let material = MeshMaterial2d(materials.add(Color::hsv(120.0, 1.0, 1.0)));
    for (entity, _) in &query {
        commands
            .entity(entity)
            .insert((shape.clone(), material.clone()));
    }
}

fn update_particle_positions(mut query: Query<(&mut Transform, &Velocity), With<Particle>>) {
    query.par_iter_mut().for_each(|(mut position, velocity)| {
        position.translation[0] += velocity.0;
        position.translation[1] += velocity.1;
    })
}

fn spawn_random_particles(mut commands: Commands) {
    let amount_to_spawn = 50;
    let spawn_radius: f32 = 400.0;
    let velocity_range = -3.0..3.0;
    let mut rng = rand::rng();
    for _ in 0..amount_to_spawn {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        let radius: f32 = rng.random_range(0.0..spawn_radius);

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let x_v = rng.random_range(velocity_range.clone());
        let y_v = rng.random_range(velocity_range.clone());
        commands.spawn((
            Particle::default(),
            Transform::from_xyz(x, y, 0.0),
            Velocity(x_v, y_v),
        ));
    }
}

#[derive(Component)]
struct Particle {
    mass: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Particle { mass: 10.0 }
    }
}

#[derive(Component)]
struct Velocity(f32, f32);
