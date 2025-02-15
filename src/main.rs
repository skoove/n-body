use bevy::prelude::*;

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
        .add_systems(Startup, (spawn_random_points, setup_camera))
        .add_systems(Update, show_points)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn show_points(
    mut commands: Commands,
    query: Query<&Position, With<Point>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for point in &query {
        let shape = Mesh2d(meshes.add(Circle::new(10.0)));
        commands.spawn((
            shape,
            MeshMaterial2d(materials.add(Color::hsv(0.0, 1.0, 1.0))),
            Transform::from_xyz(point.0, point.1, 0.0),
        ));
    }
}

fn spawn_random_points(mut commands: Commands) {
    commands.spawn((Point, Position(100.0, 100.0)));
    commands.spawn((Point, Position(-100.0, 100.0)));
    commands.spawn((Point, Position(100.0, -100.0)));
    commands.spawn((Point, Position(-100.0, -100.0)));
}

#[derive(Component)]
struct Point;

#[derive(Component)]
struct Position(f32, f32);
