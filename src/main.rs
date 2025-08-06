mod block;
mod chunk;
mod config;
mod mesher;

use bevy::{
    image::{ImageFilterMode, ImageSamplerDescriptor},
    prelude::*,
};

use bevy_rapier3d::prelude::*;

use rand::random_bool;

use block::{BlockType, Voxel};
use chunk::Chunk;
use config::CHUNK_SIZE;
use mesher::{build_mesh, generate_mesh};

use bevy_plugins::{camera::CameraPlugin, window::WindowManagerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                mag_filter: ImageFilterMode::Nearest,
                min_filter: ImageFilterMode::Nearest,
                mipmap_filter: ImageFilterMode::Linear,
                lod_min_clamp: 0.,
                lod_max_clamp: 0.,
                ..Default::default()
            },
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(WindowManagerPlugin)
        .add_plugins(VoxelPlugin)
        .run();
}

struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, voxel_setup);
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
        .insert(Transform::default())
        .insert(
            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::all())).unwrap(),
        )
        .insert(RigidBody::Fixed);

    commands
        .spawn(DirectionalLight { shadows_enabled: true, ..Default::default() })
        .insert(Transform::default().looking_at(Vec3::new(0.3, -1., 1.), Vec3::Y));

    let chunk2 = Chunk {
        voxels: [[[Voxel::Full(BlockType::Grass); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    };
    let chunk2_mesh = generate_mesh(&chunk2);
    let mesh2 = build_mesh(&chunk2_mesh);
    commands
        .spawn(Mesh3d(meshes.add(mesh2.clone())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass_texture.clone()),
            perceptual_roughness: 1.,
            reflectance: 0.03,
            cull_mode: None,
            ..Default::default()
        })))
        .insert(Transform::from_xyz(CHUNK_SIZE as f32 * 3., 0., -(CHUNK_SIZE as f32) * 3.));
}
