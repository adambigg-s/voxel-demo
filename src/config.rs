use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

pub mod blocks {
    pub const CHUNK_SIZE: usize = 32;
    pub const VOXEL_SIZE: f32 = 1.;
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
