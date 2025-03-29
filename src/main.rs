use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{CursorOptions, PresentMode};

use camera::CameraPlugin;
use gui::GuiPlugin;
use input::InputPlugin;
use particle::ParticlePlugin;
use simulation::SimPlugin;

mod camera;
mod gui;
mod input;
mod particle;
mod simulation;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "n-body".into(),
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Current,
                        ),
                        cursor_options: CursorOptions {
                            visible: true,
                            ..Default::default()
                        },
                        present_mode: PresentMode::AutoNoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..Default::default()
                }),
        )
        .add_plugins((
            CameraPlugin,
            GuiPlugin,
            InputPlugin,
            SimPlugin,
            ParticlePlugin,
        ))
        .run();
}
