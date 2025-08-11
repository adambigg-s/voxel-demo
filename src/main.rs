mod block;
mod chunk;
mod config;
mod mesher;
mod player;
mod skybox;
mod voxels;
mod world;

use bevy::prelude::*;

use voxels::VoxelPlugin;

fn main() {
    App::new().add_plugins(VoxelPlugin).run();
}
