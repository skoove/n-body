use bevy::input::mouse::MouseScrollUnit::{Line, Pixel};
use bevy::input::mouse::MouseWheel;

use crate::*;

const ZOOM_SENSITIVITY: f32 = 0.1;

pub struct CameraPlugin;

#[derive(Component)]
struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, zoom_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera, Camera2d, Transform::default()));
}

fn zoom_camera(
    mut cam_q: Query<&mut OrthographicProjection>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
) {
    let mut projection = cam_q.single_mut();
    for event in mouse_scroll_events.read() {
        match event.unit {
            Line => projection.scale += -event.y * ZOOM_SENSITIVITY,
            Pixel => projection.scale += -event.y * ZOOM_SENSITIVITY,
        }
        projection.scale = projection.scale.max(0.0);
    }
}
