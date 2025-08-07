use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::block::Voxel;
use crate::chunk::Chunk;
use crate::config::blocks::TRI_COLLIDER_MESH;
use crate::mesher::build_mesh;
use crate::mesher::generate_mesh;

pub struct WorldChunksPlugin;

impl Plugin for WorldChunksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldChunks>();
        app.add_event::<BlockBreakEvent>();
        app.add_systems(Update, chunk_break_rebuild);
    }
}

#[derive(Component)]
pub struct ChunkMarker;

#[derive(Default, Resource)]
pub struct WorldChunks {
    pub chunks: Option<Chunk>,
}

#[derive(Event)]
pub struct BlockBreakEvent {
    pub block: IVec3,
}

fn chunk_break_rebuild(
    mut break_event: EventReader<BlockBreakEvent>,
    mut world: ResMut<WorldChunks>,
    mut query: Query<(&mut Mesh3d, &mut Collider), With<ChunkMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let chunk = world.chunks.as_mut().expect("chunk never added");
    let mut changed = false;

    for event in break_event.read() {
        let pos = event.block;

        if chunk.voxels[pos.z as usize][pos.y as usize][pos.x as usize] != Voxel::Empty {
            chunk.voxels[pos.z as usize][pos.y as usize][pos.x as usize] = Voxel::Empty;
            changed = true;
        }
    }

    if changed {
        for (mut mesh, mut collider) in &mut query {
            let new_mesh = generate_mesh(chunk);
            let bevy_mesh = build_mesh(&new_mesh);

            *mesh = Mesh3d(meshes.add(bevy_mesh.clone()));
            *collider = Collider::from_bevy_mesh(&bevy_mesh, &TRI_COLLIDER_MESH)
                .expect("error regenerating chunk mesh and collider");
        }
    }
}
