use bevy::prelude::*;
use bevy::window::{CursorOptions, PresentMode};

use camera::*;
use gui::*;
use motion::*;
use particle::*;

mod camera;
mod gui;
mod motion;
mod particle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "donttilethanks".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                cursor_options: CursorOptions {
                    visible: true,
                    ..Default::default()
                },
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GuiPlugin)
        .add_plugins(MotionPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(CameraPlugin)
        .run();
}
