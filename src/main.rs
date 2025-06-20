use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{CursorOptions, PresentMode};

use n_body::camera::CameraPlugin;
use n_body::gui::GuiPlugin;
use n_body::input::InputPlugin;
use n_body::particle::ParticlePlugin;
use n_body::simulation::SimPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "n-body".into(),
                        mode: bevy::window::WindowMode::Windowed,
                        present_mode: PresentMode::AutoNoVsync,

                        cursor_options: CursorOptions {
                            visible: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..Default::default()
                }),
            FrameTimeDiagnosticsPlugin::new(100),
        ))
        .add_plugins((
            CameraPlugin,
            GuiPlugin,
            InputPlugin,
            SimPlugin,
            ParticlePlugin,
        ))
        .insert_resource(ClearColor(Color::srgb_u8(0x28, 0x28, 0x28)))
        .run();
}
