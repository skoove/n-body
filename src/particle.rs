use bevy::color::Color;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::simulation::motion::Acceleration;
use crate::simulation::motion::OldPosition;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_random_particles, show_particles).chain());
    }
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    radius: Radius,
    mass: Mass,
    position: Transform,
    old_position: OldPosition,
    acceleration: Acceleration,
}

impl ParticleBundle {
    pub fn new() -> Self {
        Self {
            particle: Particle,
            radius: Radius(1.0),
            mass: Mass(1.0),
            position: Transform::from_xyz(0.0, 0.0, 0.0),
            old_position: OldPosition(Transform::from_xyz(0.0, 0.0, 0.0)),
            acceleration: Acceleration(Vec2::ZERO),
        }
    }

    /// Spawn particle
    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(ParticleBundle {
            particle: self.particle,
            radius: self.radius,
            mass: self.mass,
            position: self.position,
            old_position: self.old_position,
            acceleration: self.acceleration,
        });
    }

    /// Set the radius of the spawned particle (visual only)
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Radius(radius);
        self
    }

    /// Set the mass of the spawned particle
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = Mass(mass);
        self
    }

    /// Set the starting position of the spawned particle
    pub fn position(mut self, pos: Vec2) -> Self {
        self.position = Transform::from_translation(pos.extend(0.0));
        self
    }

    /// Set velocity of the spawned particle, set this after position
    pub fn velocity(mut self, velo: Vec2) -> Self {
        self.old_position.0.translation = self.position.translation - velo.extend(0.0);
        self
    }
}

fn show_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &Radius, &mut Transform), With<Particle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(1.0));

    for (entity, radius, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(radius.0);
        let material_handle = materials.add(ColorMaterial::from_color(Color::hsv(0.0, 0.0, 1.0)));
        let material = MeshMaterial2d(material_handle);
        commands
            .entity(entity)
            .insert((Mesh2d(mesh.clone()), material.clone()));
    }
}

fn spawn_random_particles(mut commands: Commands) {
    let amount_to_spawn = 1000;
    let mut rng = rand::rng();
    for _ in 0..amount_to_spawn {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        let radius: f32 = rng.random_range(100.0..400.0);

        let x = radius * angle.cos();
        let y = radius * angle.sin();
        ParticleBundle::new()
            .position(Vec2::new(x, y))
            .radius(10.0)
            .velocity(Vec2::ZERO)
            .mass(10000.0)
            .spawn(&mut commands);
    }
}

pub fn set_color(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    handle: Handle<ColorMaterial>,
    color: Color,
) {
    if let Some(material) = materials.get_mut(&handle) {
        material.color = color;
        debug!("setting to {:#?}", color)
    } else {
        error!("failed to find a material: {:?}", handle)
    };
}
