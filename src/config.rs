#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

pub mod world {
    pub const RENDER_DISTANCE: usize = 0;
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
    pub const SKYBOX_SIZE: f32 = 2000.;
    pub const SUN_COLOR: bevy::color::Color = bevy::color::Color::srgb(1., 0.9, 0.9);
    pub const AMBIENT_COLOR: bevy::color::Color = bevy::color::Color::srgb(1., 0.75, 0.75);
    pub const SUN_STRENGTH: f32 = 30000.;
    pub const AMBIENT_STRENGTH: f32 = 750.;
}

pub mod keys {
    pub const RAPIER_RENDER: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyY;
    pub const CAMERA_CYCLE: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyU;
    pub const PLAYER_RESET: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyP;
    pub const WALK_FOR: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyW;
    pub const WALK_LEF: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyA;
    pub const WALK_BAC: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyS;
    pub const WALK_RIG: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyD;
    pub const WALK_UPW: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyR;
    pub const WALK_DOW: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyF;
    pub const JUMP: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::Space;
    pub const CYCLE_BLOCK_UP: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyR;
    pub const CYCLE_BLOCK_DOWN: bevy::input::keyboard::KeyCode = bevy::input::keyboard::KeyCode::KeyF;
}

pub mod player {
    pub const BLOCK_REACH: f32 = 7.5;
}
