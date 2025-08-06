#![allow(dead_code)]
#![allow(unused_imports)]

mod block;
mod chunk;
mod config;
mod mesher;
mod player;
mod world;

use bevy::{image::ImageSamplerDescriptor, prelude::*};

use bevy_rapier3d::prelude::*;

use player::PlayerPlugin;
use rand::random_bool;

use block::{BlockType, Voxel};
use chunk::Chunk;
use config::CHUNK_SIZE;
use mesher::{build_mesh, generate_mesh};

use bevy_plugins::{camera::CameraPlugin, window::WindowManagerPlugin};
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }))
        // .add_plugins(CameraPlugin)
        .add_plugins(WindowManagerPlugin)
        .add_plugins(VoxelPlugin)
        .run();
}

struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, voxel_setup);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(PlayerPlugin);
        app.add_plugins(WorldPlugin);
    }
}

fn voxel_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let grass_texture: Handle<Image> = asset_server.load("atlas.png");
    let mut chunk = Chunk::default();
    for i in 0..CHUNK_SIZE {
        for k in 0..CHUNK_SIZE {
            let height = CHUNK_SIZE as f32 / 2.
                + ((i + k) as f32 / (CHUNK_SIZE + CHUNK_SIZE) as f32) * CHUNK_SIZE as f32 / 2.;
            for j in 0..=height as usize {
                chunk.voxels[i][j][k] = Voxel::Full(BlockType::Grass);
                if j != height as usize {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Dirt);
                }
                if random_bool(0.01) {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Sand);
                }
            }
        }
    }

    let chunk_mesh = generate_mesh(&chunk);
    let mesh = build_mesh(&chunk_mesh);
    commands
        .spawn(Mesh3d(meshes.add(mesh.clone())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass_texture.clone()),
            perceptual_roughness: 1.,
            reflectance: 0.03,
            cull_mode: None,
            ..Default::default()
        })))
        .insert(
            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::empty()))
                .expect("error generating rapier collider for chunk"),
        )
        .insert(RigidBody::Fixed)
        .insert(Transform::default());

    commands
        .spawn(DirectionalLight { shadows_enabled: true, ..Default::default() })
        .insert(Transform::default().looking_at(Vec3::new(0.3, -1., 1.), Vec3::Y));
}
