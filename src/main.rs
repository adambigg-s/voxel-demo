mod block;
mod chunk;
mod config;
mod mesher;
mod player;
mod skybox;
mod world;

use bevy::{image::ImageSamplerDescriptor, prelude::*};

use bevy_rapier3d::prelude::*;

use rand::random_bool;

use noise::{NoiseFn, Perlin};

use bevy_plugins::{camera::CameraPlugin, window::WindowManagerPlugin};

use block::{BlockType, Voxel};
use chunk::Chunk;
use config::{
    blocks::{CHUNK_SIZE, TRI_COLLIDER_MESH},
    keys::{CAMERA_CYCLE, RAPIER_RENDER},
};
use mesher::{build_mesh, generate_mesh};
use player::{PlayerCamera, PlayerPlugin};
use world::{ChunkMarker, WorldChunks, WorldChunksPlugin};

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
        app.add_plugins(RapierDebugRenderPlugin { enabled: false, ..Default::default() });
        app.add_plugins(PlayerPlugin);
        app.add_plugins(WorldChunksPlugin);
        app.add_systems(Startup, voxel_setup);
        app.add_systems(Update, debug_render_toggle);
        app.add_systems(Update, debug_camera_cycle);
        app.add_systems(Update, debug_camera_fov);
    }
}

fn voxel_setup(
    mut commands: Commands,
    mut world: ResMut<WorldChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut chunk = Chunk::default();
    let perlinoc0 = Perlin::new(59930133);
    let perlinoc1 = Perlin::new(3930303);
    let perlinoc2 = Perlin::new(39989);
    let perlinoc3 = Perlin::new(541);
    for i in 0..CHUNK_SIZE {
        for k in 0..CHUNK_SIZE {
            let height_float = perlinoc0.get([i as f64 / 315., k as f64 / 315.]).abs() * CHUNK_SIZE as f64
                / 2.
                + perlinoc1.get([i as f64 / 100., k as f64 / 100.]).abs() * CHUNK_SIZE as f64 / 2.
                + perlinoc2.get([i as f64 / 32., k as f64 / 32.]).abs() * CHUNK_SIZE as f64 / 4.
                + perlinoc3.get([i as f64 / 16., k as f64 / 16.]).abs() * CHUNK_SIZE as f64 / 8.;
            let height = (height_float as usize).min(CHUNK_SIZE - 1);
            for j in 0..=height {
                if j == height {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Grass);
                }
                else if j > height.saturating_sub(3) {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Dirt);
                }
                else if random_bool(0.05) {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Coal);
                }
                else {
                    chunk.voxels[i][j][k] = Voxel::Full(BlockType::Stone);
                }
            }
        }
    }
    world.chunks = Some(chunk);

    let textures: Handle<Image> = asset_server.load("texture_atlas.png");
    let chunk_mesh = generate_mesh(world.chunks.as_ref().expect("failed to insert chunk"));
    let mesh = build_mesh(&chunk_mesh);
    commands
        .spawn(Mesh3d(meshes.add(mesh.clone())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(textures.clone()),
            perceptual_roughness: 0.9,
            reflectance: 0.001,
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
            illuminance: 30000.,
            ..Default::default()
        })
        .insert(Transform::default().looking_at(Vec3::new(0.5, -2., 1.), Vec3::Y));

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

fn debug_camera_fov(mut query: Single<&mut Projection, With<PlayerCamera>>, keys: Res<ButtonInput<KeyCode>>) {
    let Projection::Perspective(inner) = query.as_mut()
    else {
        return;
    };
    if keys.just_pressed(KeyCode::Minus) {
        inner.fov -= 0.1;
    }
    if keys.just_pressed(KeyCode::Equal) {
        inner.fov += 0.1;
    }
}
