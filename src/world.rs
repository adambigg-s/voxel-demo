use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::chunk::Chunk;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>();
    }
}

#[derive(Resource)]
struct World {
    _chunks: HashMap<IVec3, Chunk>,
}

impl Default for World {
    fn default() -> Self {
        Self { _chunks: HashMap::new() }
    }
}
