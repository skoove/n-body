use bevy::prelude::*;
use std::f32::consts::PI;
use rand::prelude::*;

use crate::{Velocity, Acceleration};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_random_particles, spawn_big_particle, show_particles).chain()
        );
    }
}

#[derive(Component)]
pub struct Particle {
    radius: f32,
}

#[derive(Component)]
pub struct Mass(pub f32);

fn show_particles(
    mut commands: Commands,
    query: Query<(Entity, &Particle)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = MeshMaterial2d(materials.add(Color::hsv(0.0, 0.0, 1.0)));
    for (entity, particle) in &query {
        let shape = Mesh2d(meshes.add(Circle::new(particle.radius)));
        commands
            .entity(entity)
            .insert((shape.clone(), material.clone()));
    }
}

fn spawn_random_particles(mut commands: Commands) {
    let amount_to_spawn = 1000;
    let velocity_range = -100.0..100.0;
    let mut rng = rand::rng();
    for _ in 0..amount_to_spawn {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        let radius: f32 = rng.random_range(100.0..400.0);

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let x_v = rng.random_range(velocity_range.clone());
        let y_v = rng.random_range(velocity_range.clone());
        commands.spawn((
            Particle { radius: 2.0 },
            Transform::from_xyz(x, y, 0.0),
            Mass(10.0),
            Velocity(Vec2::new(x_v, y_v)),
            Acceleration(Vec2::new(0.0, 0.0)),
        ));
    }
}

fn spawn_big_particle(mut commands: Commands) {
    commands.spawn((
        Particle { radius: 5.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mass(250.0),
        Velocity(Vec2::new(0.0, 0.0)),
        Acceleration(Vec2::new(0.0, 0.0)),
    ));
}
