mod block;
mod chunk;
mod config;
mod mesher;
mod player;
mod skybox;
mod world;

use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use config::blocks::TRI_COLLIDER_MESH;
use rand::random_bool;

use noise::{NoiseFn, Perlin};

use bevy_plugins::camera::CameraPlugin;
use bevy_plugins::window::WindowManagerPlugin;

use block::BlockType;
use block::Voxel;
use chunk::Chunk;
use config::blocks::CHUNK_SIZE;
use config::keys::CAMERA_CYCLE;
use config::keys::RAPIER_RENDER;
use mesher::build_mesh;
use mesher::generate_mesh;
use player::PlayerPlugin;
use world::ChunkMarker;
use world::WorldChunks;
use world::WorldChunksPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }))
        .add_plugins(CameraPlugin)
        .add_plugins(WindowManagerPlugin)
        .add_plugins(VoxelPlugin)
        .run();
}

struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(PlayerPlugin);
        app.add_plugins(WorldChunksPlugin);
        app.add_systems(Startup, voxel_setup);
        app.add_systems(Update, debug_render_toggle);
        app.add_systems(Update, debug_camera_cycle);
    }
}

fn voxel_setup(
    mut commands: Commands,
    mut world: ResMut<WorldChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let grass_texture: Handle<Image> = asset_server.load("atlas.png");
    let mut chunk = Chunk::default();
    let perlin = Perlin::new(256);
    for i in 0..CHUNK_SIZE {
        for k in 0..CHUNK_SIZE {
            let height_float = perlin.get([i as f64 / 128., k as f64 / 128.]).abs() * CHUNK_SIZE as f64;
            let height = (height_float as usize).min(CHUNK_SIZE - 1);
            for j in 0..=height {
                if j == height {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Grass);
                }
                else if random_bool(0.03) {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Sand);
                }
                else {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Dirt);
                }
            }
        }
    }
    world.chunks = Some(chunk);

    let chunk_mesh = generate_mesh(world.chunks.as_ref().expect("failed to insert chunk"));
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
        .insert(ChunkMarker)
        .insert(
            Collider::from_bevy_mesh(&mesh, &TRI_COLLIDER_MESH)
                .expect("error generating rapier collider for chunk"),
        )
        .insert(RigidBody::Fixed)
        .insert(Transform::default());

    commands
        .spawn(DirectionalLight {
            color: Color::srgb(1., 0.9, 0.9),
            shadows_enabled: true,
            illuminance: 50000.,
            ..Default::default()
        })
        .insert(Transform::default().looking_at(Vec3::new(0.3, -1., 1.), Vec3::Y));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(1., 0.75, 0.75),
        brightness: 750.,
        ..Default::default()
    });
}

fn debug_render_toggle(mut render: ResMut<DebugRenderContext>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(RAPIER_RENDER) {
        render.enabled = !render.enabled;
    }
}

fn debug_camera_cycle(mut query: Query<&mut Camera, With<Camera3d>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(CAMERA_CYCLE) {
        for mut camera in &mut query {
            camera.is_active = !camera.is_active;
        }
    }
}
