use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

pub mod blocks {
    use bevy_rapier3d::prelude::{ComputedColliderShape, TriMeshFlags};
    pub const CHUNK_SIZE: usize = 32;
    pub const VOXEL_SIZE: f32 = 1.;
    pub const TRI_COLLIDER_MESH: ComputedColliderShape =
        ComputedColliderShape::TriMesh(TriMeshFlags::ORIENTED);
}

pub mod aesthetics {
    pub const ATLAS_SIZE: usize = 320;
    pub const TEXTURE_SIZE: usize = 32;
}

pub mod keys {
    use super::*;
    pub const RAPIER_RENDER: KeyCode = KeyCode::KeyY;
    pub const CAMERA_CYCLE: KeyCode = KeyCode::KeyU;
    pub const PLAYER_RESET: KeyCode = KeyCode::KeyP;
}

pub mod player {
    pub const BLOCK_REACH: f32 = 4.;
}
