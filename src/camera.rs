use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::PrimaryWindow,
};

const CAMERA_ZOOM_SENSE: f32 = 0.3;
const CAMERA_ZOOM_FLOATYNESS: f32 = 10.0;
const CAMERA_PAN_FLOATYNESS: f32 = 10.0;

/// Provides a nice 2d camera
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

#[derive(Component)]
struct CameraTarget {
    position: Vec3,
    zoom: f32,
}

impl Default for CameraTarget {
    fn default() -> Self {
        Self {
            position: Default::default(),
            zoom: 0.5,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default(), CameraTarget::default()));
}

fn zoom_camera(
    mouse_wheel: Res<AccumulatedMouseScroll>,
    camera_query: Single<(&mut Projection, &mut CameraTarget), With<Camera>>,
    time: Res<Time>,
) -> Result {
    let (projection, mut camera_target) = camera_query.into_inner();

    let projection = match projection.into_inner() {
        Projection::Orthographic(projection) => projection,
        _ => {
            return Err("only orthographic projections are supported!".into());
        }
    };

    if (camera_target.zoom - projection.scale).abs() >= 0.01 {
        projection.scale = projection.scale.lerp(
            camera_target.zoom,
            CAMERA_ZOOM_FLOATYNESS * time.delta_secs(),
        )
    }

    // -= so taht it goes in correct direction
    camera_target.zoom -= mouse_wheel.delta.y * (camera_target.zoom * CAMERA_ZOOM_SENSE);
    camera_target.zoom = camera_target.zoom.max(0.01);

    Ok(())
}

fn pan_camera(
    camera_query: Single<(&mut Transform, &mut CameraTarget, &Projection), With<Camera>>,
    mouse_movement: Res<AccumulatedMouseMotion>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) -> Result {
    let (mut transform, mut camera_target, projection) = camera_query.into_inner();

    let projection = match projection {
        Projection::Orthographic(projection) => projection,
        _ => {
            return Err("only orthographic projections are supported!".into());
        }
    };

    if (camera_target.position - transform.translation).length() >= 0.1 {
        transform.translation = transform.translation.lerp(
            camera_target.position,
            CAMERA_PAN_FLOATYNESS * time.delta_secs(),
        );
    }

    if mouse_button_events.pressed(MouseButton::Right) {
        camera_target.position +=
            mouse_movement.delta.reflect(Vec2::X).extend(0.0) * projection.scale;
    }

    Ok(())
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
