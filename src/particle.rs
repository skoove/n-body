use bevy::color::Color;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::simulation::motion::Acceleration;
use crate::simulation::motion::OldPosition;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_random_particles)
            .add_systems(Update, show_particles);
    }
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Mass(pub f32);

/// The color **at spawn** of an entity. To change color use [`set_color()`]
#[derive(Component)]
pub struct SpawnColor(bevy::color::Color);

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    radius: Radius,
    mass: Mass,
    position: Transform,
    old_position: OldPosition,
    acceleration: Acceleration,
    color: SpawnColor,
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
            color: SpawnColor(Color::hsv(0.0, 0.0, 1.00)),
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
            color: self.color,
        });
    }

    /// Set the radius of the spawned particle
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

    /// Set the color of the spawned particle
    pub fn color(mut self, color: Color) -> Self {
        self.color = SpawnColor(color);
        self
    }
}

#[allow(clippy::type_complexity)] // sorry clippy i kinda need it
fn show_particles(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Radius, &mut Transform, &SpawnColor),
        (
            With<Particle>,
            Without<MeshMaterial2d<ColorMaterial>>,
            Without<Mesh2d>,
        ),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(1.0));

    for (entity, radius, mut transform, SpawnColor(color)) in query.iter_mut() {
        transform.scale = Vec3::splat(radius.0);
        let material_handle = materials.add(ColorMaterial::from_color(*color));
        let material = MeshMaterial2d(material_handle);
        commands
            .entity(entity)
            .insert((Mesh2d(mesh.clone()), material.clone()));
    }
}

fn spawn_random_particles(mut commands: Commands) {
    let amount_to_spawn = 500;
    let mut rng = rand::rng();
    for i in 0..amount_to_spawn {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        let radius: f32 = rng.random_range(500.0..2000.0);

        let velo_range = -5.0..5.0;
        let velo_x: f32 = rng.random_range(velo_range.clone());
        let velo_y: f32 = rng.random_range(velo_range.clone());
        let velo: Vec2 = Vec2::new(velo_x, velo_y);

        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let pos = Vec2::new(x, y);
        ParticleBundle::new()
            .position(pos)
            .radius(10.0)
            .velocity(velo)
            .mass(1000.0)
            .color(Color::hsv(
                (i as f32 / amount_to_spawn as f32) * 30.0,
                1.0,
                1.0,
            ))
            .spawn(&mut commands);
    }
}

/// Set the colour of a given [ColorMaterial]
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

#[cfg(test)]
mod tests {
    fn build_test_particle() -> super::ParticleBundle {
        use super::ParticleBundle;
        use bevy::prelude::*;

        ParticleBundle::new()
            .position(Vec2::new(100.0, 200.0))
            .velocity(Vec2::new(5.0, 10.0))
            .radius(15.0)
            .mass(500.0)
            .color(Color::srgb(0.5, 0.5, 0.5))
    }

    #[test]
    fn test_particle_bundle_builder() {
        use super::SpawnColor;
        use bevy::prelude::*;

        let pos = Vec2::new(100.0, 200.0);
        let velo = Vec2::new(5.0, 10.0);

        let bundle = build_test_particle();

        assert_eq!(bundle.position.translation, pos.extend(0.0));
        // old position should be position - velocity
        let expected_old = bundle.position.translation - velo.extend(0.0);
        assert_eq!(bundle.old_position.0.translation, expected_old);
        assert_eq!((bundle.radius).0, 15.0);
        assert_eq!((bundle.mass).0, 500.0);
        let SpawnColor(col) = bundle.color;
        assert_eq!(col, Color::srgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_normal_particle_bundle_spawning() {
        use super::Particle;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, spawn);

        fn spawn(mut commands: Commands) {
            build_test_particle().spawn(&mut commands);
        }

        app.update();

        let particle_count = app.world_mut().query::<&Particle>().iter(app.world()).len();
        assert_eq!(particle_count, 1);
    }

    #[test]
    fn test_negitive_radius_particle_bundle_spawning() {
        use super::Particle;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, spawn);

        fn spawn(mut commands: Commands) {
            build_test_particle().radius(-10.0).spawn(&mut commands);
        }

        app.update();

        let particle_count = app.world_mut().query::<&Particle>().iter(app.world()).len();
        assert_eq!(particle_count, 1);
    }

    #[test]
    fn test_negitive_mass_particle_bundle_spawning() {
        use super::Particle;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, spawn);

        fn spawn(mut commands: Commands) {
            build_test_particle().mass(-10.0).spawn(&mut commands);
        }

        app.update();

        let particle_count = app.world_mut().query::<&Particle>().iter(app.world()).len();
        assert_eq!(particle_count, 1);
    }

    #[test]
    fn test_set_color() {
        use super::set_color;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, update);
        app.insert_resource(Assets::<ColorMaterial>::default());
        app.update();

        fn update(mut materials: ResMut<Assets<ColorMaterial>>) {
            let handle = materials.add(ColorMaterial::from(Color::WHITE));
            set_color(&mut materials, handle.clone(), Color::BLACK);
            assert_eq!(materials.get(&handle).unwrap().color, Color::BLACK);
        }
    }
}
