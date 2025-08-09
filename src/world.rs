use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::block::Voxel;
use crate::chunk::Chunk;
use crate::config::blocks::TRI_COLLIDER_MESH;
use crate::config::world::RENDER_DISTANCE;
use crate::mesher::build_mesh;
use crate::mesher::generate_mesh;

pub struct WorldChunksPlugin;

impl Plugin for WorldChunksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldChunks>();
        app.add_event::<BlockBreakEvent>();
        app.add_event::<BlockPlaceEvent>();
        app.add_systems(Update, chunk_block_break);
        app.add_systems(Update, chunk_block_place);
    }
}

#[derive(Component)]
pub struct ChunkMarker;

#[derive(Default, Resource)]
pub struct WorldChunks {
    pub chunks: Option<Chunk>,
}

#[derive(Resource)]
pub struct _WorldChunksInfinite {
    pub chunks: HashMap<IVec3, Chunk>,
}

impl Default for _WorldChunksInfinite {
    fn default() -> Self {
        Self {
            chunks: HashMap::with_capacity(RENDER_DISTANCE * RENDER_DISTANCE),
        }
    }
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

fn chunk_block_break(
    mut break_event: EventReader<BlockBreakEvent>,
    mut world: ResMut<WorldChunks>,
    query: Query<(&mut Mesh3d, &mut Collider), With<ChunkMarker>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let chunk = world.chunks.as_mut().expect("chunk never added");
    let mut changed = false;

    for event in break_event.read() {
        let [x, y, z] = event.position.to_array().map(|value| value as usize);

        if chunk.voxels[z][y][x] != Voxel::Empty {
            chunk.voxels[z][y][x] = Voxel::Empty;
            changed = true;
        }
    }

    if changed {
        chunk_rebuild(query, meshes, chunk);
    }
}

fn chunk_block_place(
    mut break_event: EventReader<BlockPlaceEvent>,
    mut world: ResMut<WorldChunks>,
    query: Query<(&mut Mesh3d, &mut Collider), With<ChunkMarker>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let chunk = world.chunks.as_mut().expect("chunk never added");
    let mut changed = false;

    for event in break_event.read() {
        let [x, y, z] = event.position.to_array().map(|value| value as usize);

        if chunk.voxels[z][y][x] == Voxel::Empty {
            chunk.voxels[z][y][x] = event.species;
            changed = true;
        }
    }

    if changed {
        chunk_rebuild(query, meshes, chunk);
    }
}

fn chunk_rebuild(
    mut query: Query<(&mut Mesh3d, &mut Collider), With<ChunkMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk: &mut Chunk,
) {
    for (mut mesh, mut collider) in &mut query {
        let new_mesh = generate_mesh(chunk);
        let bevy_mesh = build_mesh(&new_mesh);

        *mesh = Mesh3d(meshes.add(bevy_mesh.clone()));
        *collider = Collider::from_bevy_mesh(&bevy_mesh, &TRI_COLLIDER_MESH)
            .expect("error regenerating chunk mesh and collider");
    }
}
