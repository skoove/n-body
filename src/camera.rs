use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::*;

const ZOOM_SENSITIVITY: f32 = 0.1;

pub struct CameraPlugin;

#[derive(Component)]
struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (zoom_camera, pan_camera));
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
        projection.scale += -event.y * ZOOM_SENSITIVITY * projection.scale;
    }
    projection.scale = projection.scale.max(0.0)
}

fn pan_camera(
    mut cam_q: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    mut mouse_movement_events: EventReader<MouseMotion>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_events.pressed(MouseButton::Right) {
        let (mut camera_transform, projection) = cam_q.single_mut();

        for event in mouse_movement_events.read() {
            let new_pos = camera_transform.translation.truncate()
                + (event.delta.with_x(-event.delta.x) * projection.scale);
            camera_transform.translation = new_pos.extend(0.0);
        }
    }
}
