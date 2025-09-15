use bevy::{prelude::*, render::mesh::CircleMeshBuilder};

use crate::particle::{Particle, Radius};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_particle_mesh_and_material)
            .add_systems(Update, give_particles_materials);
    }
}

#[derive(Resource)]
pub struct ParticleColorMaterial(Handle<ColorMaterial>);

#[derive(Resource)]
pub struct ParticleMesh(Handle<Mesh>);

fn init_particle_mesh_and_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = ColorMaterial::from_color(Color::srgb(1.0, 1.0, 1.0));
    let mesh = CircleMeshBuilder::new(1.0, 20);
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);
    commands.insert_resource(ParticleMesh(mesh_handle));
    commands.insert_resource(ParticleColorMaterial(material_handle));
}

fn give_particles_materials(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Radius, &mut Transform),
        (
            With<Particle>,
            Without<MeshMaterial2d<ColorMaterial>>,
            Without<Mesh2d>,
        ),
    >,
    mesh: Res<ParticleMesh>,
    material: Res<ParticleColorMaterial>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, radius, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(radius.0);
        let material = MeshMaterial2d(material.0.clone());
        let mesh = Mesh2d(mesh.0.clone());
        commands.entity(entity).insert((mesh, material));
    }
}
