use std::f32::consts::PI;

use bevy::prelude::*;
use rand::random_range;

use super::ParticleBundle;

pub struct SpawnRandomParticles {
    amount: u32,
    outer_radius: f32,
    inner_radius: f32,
    radius: f32,
    mass: f32,
    velocity_range: f32,
    value_variation: bool,
    position: Vec2,
}

impl SpawnRandomParticles {
    /// Create new random particle spawner, call spawn to actually spawn it
    pub fn new() -> Self {
        Self {
            amount: 100,
            outer_radius: 100.0,
            inner_radius: 0.0,
            value_variation: false,
            radius: 1.0,
            mass: 1.0,
            velocity_range: 0.0,
            position: Vec2::ZERO,
        }
    }

    /// The amount of particles to spawn
    pub fn amount(mut self, amount: u32) -> Self {
        self.amount = amount;
        self
    }

    /// The mass of the spawned particles
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    /// The maximum velocity of spawned particles
    pub fn velocity(mut self, velocity_range: f32) -> Self {
        self.velocity_range = velocity_range;
        self
    }

    /// The outer radius of the circle that the particles will be spawned in
    pub fn outer_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius = outer_radius;
        self
    }

    /// The inner radius of the circle that the particles will be spawned in
    pub fn inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius;
        self
    }

    /// If true, all the particles values will be different, making them appear a bit different
    pub fn value_variation(mut self, value_variation: bool) -> Self {
        self.value_variation = value_variation;
        self
    }

    /// The center of the circle for particles to be spawned in
    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    /// The radius of the spawned particles
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Spawn the particles
    pub fn spawn(self, commands: &mut Commands) {
        for i in 0..self.amount {
            let mut value = 1.0;
            if self.value_variation {
                value = (i as f32 / self.amount as f32) * 0.8 + 0.2;
            }
            let angle = random_range(0.0..2.0 * PI);
            let distance = random_range(self.inner_radius..self.outer_radius);
            let position = self.position + Vec2::from_angle(angle) * distance;
            let mut velocity = Vec2::ZERO;

            if self.velocity_range != 0.0 {
                let velo_angle = random_range(0.0..2.0 * PI);
                let velo = random_range(0.0..=self.velocity_range);
                velocity = Vec2::from_angle(velo_angle) * velo;
            }

            ParticleBundle::new()
                .radius(self.radius)
                .color(Color::hsv(0.0, 0.0, value))
                .position(position)
                .velocity(velocity)
                .mass(self.mass)
                .spawn(commands);
        }
    }
}
