use core::panic;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

const ZOOM_DAMPING: f32 = 5.0;
const ZOOM_SENSITIVITY: f32 = 3.0;

const PAN_DAMPING: f32 = 5.0;
const PAN_SENSITIVITY: f32 = 10.0;

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

#[derive(Component, Default)]
struct CameraVelocity {
    position: Vec3,
    zoom: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default(), CameraVelocity::default()));
}

fn zoom_camera(
    mouse_wheel: Res<AccumulatedMouseScroll>,
    camera_query: Single<(&mut Projection, &mut CameraVelocity), With<Camera>>,
    time: Res<Time>,
) {
    let (mut projection, mut velocity) = camera_query.into_inner();
    match *projection {
        Projection::Orthographic(ref mut orthographic) => {
            velocity.zoom += mouse_wheel.delta.y * ZOOM_SENSITIVITY;
            velocity.zoom *= (1.0 - ZOOM_DAMPING * time.delta_secs()).max(0.0);
            orthographic.scale -= orthographic.scale * velocity.zoom * time.delta_secs();
            orthographic.scale = orthographic.scale.max(0.01)
        }
        _ => (),
    }
}

fn pan_camera(
    camera_query: Single<(&mut Transform, &Projection, &mut CameraVelocity), With<Camera>>,
    mouse_movement: Res<AccumulatedMouseMotion>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let (mut transform, projection, mut velocity) = camera_query.into_inner();

    let projection = match projection {
        Projection::Orthographic(projection) => projection,
        _ => {
            error!("only orthographic projections are supported!");
            return ();
        }
    };

    if mouse_button_events.pressed(MouseButton::Right) {
        let delta = mouse_movement.delta.reflect(Vec2::X).extend(0.0) * projection.scale;
        velocity.position += delta * PAN_SENSITIVITY;
    }

    transform.translation += velocity.position * time.delta_secs();

    velocity.position *= (1.0 - PAN_DAMPING * time.delta_secs()).max(0.0);
}

/// stolen from <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
/// modifed for bevy 1.16
fn get_world_coords(
    mut mycoords: ResMut<CursorWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Single<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = *q_camera;

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window;

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
