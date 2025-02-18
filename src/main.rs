use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use camera::*;
use motion::*;
use particle::*;

mod camera;
mod motion;
mod particle;

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
        .add_plugins(MotionPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(CameraPlugin)
        .run();
}
