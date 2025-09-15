use bevy::prelude::*;

use crate::simulation;
use crate::simulation::motion::Acceleration;
use crate::simulation::motion::OldPosition;
use crate::simulation::motion::PreviousAcceleration;

pub mod spawners;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, count_particles)
            .add_systems(
                FixedUpdate,
                (spawners::particle_hose_system).run_if(simulation::sim_not_paused),
            )
            .init_resource::<ParticleCount>();
    }
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component, Clone, Copy, Debug)]
pub struct Mass(pub f32);

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    radius: Radius,
    mass: Mass,
    position: Transform,
    old_position: OldPosition,
    acceleration: Acceleration,
    previous_acceleration: PreviousAcceleration,
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
            previous_acceleration: PreviousAcceleration(Vec2::ZERO),
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
            previous_acceleration: self.previous_acceleration,
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
}

#[derive(Resource, Default)]
pub struct ParticleCount(pub usize);

fn count_particles(particles: Query<&Particle>, mut particle_count: ResMut<ParticleCount>) {
    particle_count.0 = particles.iter().count();
}

pub fn despawn_particles(commands: &mut Commands, particles: Query<Entity, With<Particle>>) {
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
    }

    #[test]
    fn test_particle_bundle_builder() {
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
    fn test_particle_despawn() {
        use super::Particle;
        use bevy::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, (spawn, despawn).chain());

        fn spawn(mut commands: Commands) {
            build_test_particle().spawn(&mut commands);
        }

        fn despawn(mut commands: Commands, particles: Query<Entity, With<Particle>>) {
            super::despawn_particles(&mut commands, particles);
        }

        app.update();

        let particle_count = app.world_mut().query::<&Particle>().iter(app.world()).len();
        assert_eq!(particle_count, 0);
    }
}
