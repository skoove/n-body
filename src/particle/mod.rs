use bevy::color::Color;
use bevy::prelude::*;

use crate::simulation::motion::Acceleration;
use crate::simulation::motion::OldPosition;

mod spawners;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (show_particles, count_particles))
            .init_resource::<ParticleCount>();
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
    /// default: 1.0
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Radius(radius);
        self
    }

    /// Set the mass of the spawned particle
    /// default: 1.0
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = Mass(mass);
        self
    }

    /// Set the starting position of the spawned particle
    /// default: 0.0 , 0.0
    pub fn position(mut self, pos: Vec2) -> Self {
        let transform = Transform::from_translation(pos.extend(0.0));
        self.position = transform;
        self.old_position = OldPosition(transform);
        self
    }

    /// Set velocity of the spawned particle, set this after position
    /// default: 0.0 , 0.0
    pub fn velocity(mut self, velo: Vec2) -> Self {
        self.old_position.0.translation = self.position.translation - velo.extend(0.0);
        self
    }

    /// Set the color of the spawned particle
    /// default: white
    #[allow(dead_code)]
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

/// Set the colour of a given [ColorMaterial]
#[allow(dead_code)] // it will definitly be used at some point
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

#[derive(Resource, Default)]
pub struct ParticleCount(pub usize);

fn count_particles(particles: Query<&Particle>, mut particle_count: ResMut<ParticleCount>) {
    particle_count.0 = particles.iter().count();
}

pub fn despawn_particles(mut commands: Commands, particles: Query<Entity, With<Particle>>) {
    for entity in particles.iter() {
        commands.entity(entity).despawn();
    }
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

    #[test]
    fn test_particle_despawn() {
        use super::Particle;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, (spawn, despawn).chain());

        fn spawn(mut commands: Commands) {
            build_test_particle().spawn(&mut commands);
        }

        fn despawn(commands: Commands, particles: Query<Entity, With<Particle>>) {
            super::despawn_particles(commands, particles);
        }

        app.update();

        let particle_count = app.world_mut().query::<&Particle>().iter(app.world()).len();
        assert_eq!(particle_count, 0);
    }
}
