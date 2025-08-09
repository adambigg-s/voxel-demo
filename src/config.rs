#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

pub mod world {
    pub const RENDER_DISTANCE: usize = 3;
}

pub mod blocks {
    pub const CHUNK_SIZE: usize = 32;
    pub const VOXEL_SIZE: f32 = 1.;
    pub const TRI_COLLIDER_MESH: bevy_rapier3d::prelude::ComputedColliderShape =
        bevy_rapier3d::prelude::ComputedColliderShape::TriMesh(
            bevy_rapier3d::prelude::TriMeshFlags::ORIENTED,
        );
}

pub mod aesthetics {
    pub const ATLAS_SIZE: usize = 256;
    pub const TEXTURE_SIZE: usize = 16;
}

pub mod keys {
    pub const RAPIER_RENDER: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyY;
    pub const CAMERA_CYCLE: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyU;
    pub const PLAYER_RESET: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyP;
}

pub mod player {
    pub const BLOCK_REACH: f32 = 5.;
}
