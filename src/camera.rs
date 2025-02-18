use crate::*;

pub struct CameraPlugin;

#[derive(Component)]
struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, pan_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera, Camera2d, Transform::default()));
}

fn pan_camera(mut cam_q: Query<(&mut Transform, &mut OrthographicProjection)>) {
    let (mut transform, mut camera) = cam_q.single_mut();
    transform.translation += Vec3::new(0.0, 0.0, 0.0);
    camera.scale -= 0.1;
}
