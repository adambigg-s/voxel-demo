use std::collections::HashMap;
use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use bevy_rapier3d::prelude::RigidBody;
use noise::NoiseFn;
use noise::Perlin;
use rand::random_bool;

use crate::block::BlockType;
use crate::block::Voxel;
use crate::chunk::Chunk;
use crate::config::blocks::CHUNK_SIZE;
use crate::config::blocks::TRI_COLLIDER_MESH;
use crate::config::world::RENDER_DISTANCE;
use crate::mesher::build_mesh;
use crate::mesher::generate_mesh;
use crate::player::Player;

pub struct WorldChunksPlugin;

impl Plugin for WorldChunksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldChunks>();
        app.init_resource::<BlockMaterial>();
        app.init_resource::<TerrainNoise>();
        app.add_event::<BlockBreakEvent>();
        app.add_event::<BlockPlaceEvent>();
        app.add_systems(Startup, chunk_startup_load);
        app.add_systems(Update, chunk_block_break);
        app.add_systems(Update, chunk_block_place);
        app.add_systems(Update, chunk_load_manager);
        app.add_systems(Update, chunk_delete_manager);
    }
}

#[derive(Default, Resource)]
struct TerrainNoise {
    noise: Perlin,
}

#[derive(Resource, Default)]
struct BlockMaterial {
    material: Handle<StandardMaterial>,
}

#[derive(Debug, Component)]
pub struct ChunkMarker {
    pub location: IVec3,
}

#[derive(Default, Resource)]
pub struct WorldChunks {
    pub chunks: HashMap<IVec3, Chunk>,
}

#[derive(Event)]
pub struct BlockBreakEvent {
    pub position: IVec3,
}

#[derive(Event)]
pub struct BlockPlaceEvent {
    pub position: IVec3,
    pub species: Voxel,
}

fn chunk_startup_load(
    mut commands: Commands,
    mut world: ResMut<WorldChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut block_material: ResMut<BlockMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_noise: Res<TerrainNoise>,
    asset_server: Res<AssetServer>,
) {
    let textures: Handle<Image> = asset_server.load("texture_atlas.png");
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(textures.clone()),
        perceptual_roughness: 0.9,
        reflectance: 0.001,
        cull_mode: None,
        ..Default::default()
    });
    block_material.material = material;

    for x in -(RENDER_DISTANCE as isize)..=RENDER_DISTANCE as isize {
        for z in -(RENDER_DISTANCE as isize)..=RENDER_DISTANCE as isize {
            let chunk_pos = IVec3::new(x as i32, 0, z as i32);

            let chunk = generate_chunk(chunk_pos, &terrain_noise);

            let mesh_builder = generate_mesh(&chunk);
            let mesh = build_mesh(&mesh_builder);
            let collider = Collider::from_bevy_mesh(&mesh, &TRI_COLLIDER_MESH)
                .expect("failed to generate rapier collider for chunk mesh");
            let transform =
                Transform::from_xyz(x as f32 * CHUNK_SIZE as f32, 0., z as f32 * CHUNK_SIZE as f32);

            world.chunks.insert(chunk_pos, chunk);

            commands
                .spawn(ChunkMarker { location: chunk_pos })
                .insert(Mesh3d(meshes.add(mesh)))
                .insert(MeshMaterial3d(block_material.material.clone()))
                .insert(collider)
                .insert(RigidBody::Fixed)
                .insert(transform);
        }
    }
}

fn generate_chunk(position: IVec3, noise: &TerrainNoise) -> Chunk {
    let mut chunk = Chunk::default();
    let noise = noise.noise;

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            let [i, k] = [
                position.x as f64 * CHUNK_SIZE as f64 + local_x as f64,
                position.z as f64 * CHUNK_SIZE as f64 + local_z as f64,
            ];
            let height_float = noise.get([i / 315., k / 315.]).abs() * CHUNK_SIZE as f64 / 2.
                + noise.get([i / 100., k / 100.]).abs() * CHUNK_SIZE as f64 / 2.
                + noise.get([i / 32., k / 32.]).abs() * CHUNK_SIZE as f64 / 4.
                + noise.get([i / 16., k / 16.]).abs() * CHUNK_SIZE as f64 / 8.;

            let height = (height_float as usize).min(CHUNK_SIZE - 1);
            for local_y in 0..=height {
                if local_y == height {
                    chunk.voxels[local_z][local_y][local_x] = Voxel::Full(BlockType::Grass);
                }
                else if local_y > height.saturating_sub(3) {
                    chunk.voxels[local_z][local_y][local_x] = Voxel::Full(BlockType::Dirt);
                }
                else if random_bool(0.05) {
                    chunk.voxels[local_z][local_y][local_x] = Voxel::Full(BlockType::Coal);
                }
                else {
                    chunk.voxels[local_z][local_y][local_x] = Voxel::Full(BlockType::Stone);
                }
            }
        }
    }

    chunk
}

fn chunk_block_break(
    mut break_event: EventReader<BlockBreakEvent>,
    mut world: ResMut<WorldChunks>,
    query: Query<(&mut Mesh3d, &mut Collider, &ChunkMarker)>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let mut changed_chunks = HashSet::new();

    for event in break_event.read() {
        let world_position = WorldPosition::get(event.position);
        let Some(chunk) = world.chunks.get_mut(&world_position.chunk_location)
        else {
            error!("chunk was not loaded yet");
            return;
        };

        let [x, y, z] = world_position.location_in_chunk.to_array().map(|value| value as usize);
        if chunk.voxels[z][y][x] != Voxel::Empty {
            chunk.voxels[z][y][x] = Voxel::Empty;
            changed_chunks.insert(world_position.chunk_location);
        }
    }

    chunk_mesh_rebuild(world, query, meshes, changed_chunks);
}

fn chunk_block_place(
    mut place_event: EventReader<BlockPlaceEvent>,
    mut world: ResMut<WorldChunks>,
    query: Query<(&mut Mesh3d, &mut Collider, &ChunkMarker)>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let mut changed_chunks = HashSet::new();

    for event in place_event.read() {
        let world_position = WorldPosition::get(event.position);
        let Some(chunk) = world.chunks.get_mut(&world_position.chunk_location)
        else {
            error!("chunk was not loaded yet");
            return;
        };

        let [x, y, z] = world_position.location_in_chunk.to_array().map(|value| value as usize);
        if chunk.voxels[z][y][x] == Voxel::Empty {
            chunk.voxels[z][y][x] = event.species;
            changed_chunks.insert(world_position.chunk_location);
        }
    }

    chunk_mesh_rebuild(world, query, meshes, changed_chunks);
}

fn chunk_mesh_rebuild(
    world: ResMut<WorldChunks>,
    mut query: Query<(&mut Mesh3d, &mut Collider, &ChunkMarker)>,
    mut meshes: ResMut<Assets<Mesh>>,
    changed_chunks: HashSet<IVec3>,
) {
    for chunk_pos in changed_chunks {
        if let Some(chunk) = world.chunks.get(&chunk_pos) {
            for (mut mesh, mut collider, marker) in &mut query {
                if marker.location != chunk_pos {
                    continue;
                }

                let new_mesh = generate_mesh(chunk);
                let bevy_mesh = build_mesh(&new_mesh);
                *mesh = Mesh3d(meshes.add(bevy_mesh.clone()));
                *collider = Collider::from_bevy_mesh(&bevy_mesh, &TRI_COLLIDER_MESH)
                    .expect("error regenerating chunk mesh and collider");
            }
        }
    }
}

fn chunk_load_manager(
    mut commands: Commands,
    mut world: ResMut<WorldChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Query<(Entity, &ChunkMarker)>,
    block_material: Res<BlockMaterial>,
    terrain_noise: Res<TerrainNoise>,
    player: Single<&Transform, With<Player>>,
) {
    let player_pos = WorldPosition::get(player.translation.as_ivec3()).chunk_location;

    let existing_chunks: HashSet<IVec3> = chunks.iter().map(|(.., marker)| marker.location).collect();

    for x in (player_pos.x - RENDER_DISTANCE as i32)..=(player_pos.x + RENDER_DISTANCE as i32) {
        for z in (player_pos.z - RENDER_DISTANCE as i32)..=(player_pos.z + RENDER_DISTANCE as i32) {
            let chunk_pos = IVec3::new(x, 0, z);

            if existing_chunks.contains(&chunk_pos) {
                continue;
            }

            world.chunks.entry(chunk_pos).or_insert_with(|| generate_chunk(chunk_pos, &terrain_noise));

            let chunk = world.chunks.get(&chunk_pos).expect("failed to insert generated chunk");

            let mesh_builder = generate_mesh(chunk);
            let mesh = build_mesh(&mesh_builder);
            let collider = Collider::from_bevy_mesh(&mesh, &TRI_COLLIDER_MESH)
                .expect("failed to generate rapier collider for chunk mesh");
            let transform =
                Transform::from_xyz(x as f32 * CHUNK_SIZE as f32, 0., z as f32 * CHUNK_SIZE as f32);

            commands
                .spawn(ChunkMarker { location: chunk_pos })
                .insert(Mesh3d(meshes.add(mesh)))
                .insert(MeshMaterial3d(block_material.material.clone()))
                .insert(collider)
                .insert(RigidBody::Fixed)
                .insert(transform);
        }
    }
}

fn chunk_delete_manager(
    mut commands: Commands,
    chunks: Query<(Entity, &ChunkMarker)>,
    player: Single<&Transform, With<Player>>,
) {
    let player_pos = WorldPosition::get(player.translation.as_ivec3()).chunk_location;

    let mut should_exist = HashSet::new();
    for x in (player_pos.x - RENDER_DISTANCE as i32)..=(player_pos.x + RENDER_DISTANCE as i32) {
        for z in (player_pos.z - RENDER_DISTANCE as i32)..=(player_pos.z + RENDER_DISTANCE as i32) {
            should_exist.insert(IVec3::new(x, 0, z));
        }
    }

    for (entity, chunk_position) in chunks {
        if !should_exist.contains(&chunk_position.location) {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Debug)]
struct WorldPosition {
    chunk_location: IVec3,
    location_in_chunk: IVec3,
}

impl WorldPosition {
    fn get(world: IVec3) -> Self {
        Self {
            chunk_location: world.div_euclid(IVec3::splat(CHUNK_SIZE as i32)),
            location_in_chunk: world.rem_euclid(IVec3::splat(CHUNK_SIZE as i32)),
        }
    }
}

#[cfg(test)]
mod world_position {
    use super::*;

    #[test]
    fn world_pos_positive() {
        let position = IVec3::new(1, 0, 0);
        let world = WorldPosition::get(position);
        dbg!(&world);
        assert!(world.chunk_location == IVec3::new(0, 0, 0));
        assert!(world.location_in_chunk == IVec3::new(1, 0, 0));
    }

    #[test]
    fn world_pos_negative() {
        let position = IVec3::new(-1, 0, 0);
        let world = WorldPosition::get(position);
        dbg!(&world);
        assert!(world.chunk_location == IVec3::new(-1, 0, 0));
        assert!(world.location_in_chunk == IVec3::new(CHUNK_SIZE as i32 - 1, 0, 0));
    }
}
