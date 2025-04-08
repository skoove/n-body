use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

const ZOOM_SENSITIVITY: f32 = 0.1;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (zoom_camera, pan_camera, get_world_coords));
        app.init_resource::<CursorWorldCoords>();
    }
}

#[derive(Resource, Default, Clone, Copy)]
/// Resource that provides the current world coords of the camera
pub struct CursorWorldCoords(pub Vec2);

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));
}

fn zoom_camera(
    mut cam_q: Query<&mut OrthographicProjection>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
) {
    let mut projection = cam_q.single_mut();
    for event in mouse_scroll_events.read() {
        projection.scale += -event.y * ZOOM_SENSITIVITY * projection.scale;
    }
    projection.scale = projection.scale.max(0.0);
}

fn pan_camera(
    mut cam_q: Query<(&mut Transform, &OrthographicProjection), With<Camera2d>>,
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

/// stolen from <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
fn get_world_coords(
    mut mycoords: ResMut<CursorWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}
