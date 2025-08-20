use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;

use bevy_plugins::window::WindowManagerPlugin;

use bevy_rapier3d::plugin::RapierPhysicsPlugin;
use bevy_rapier3d::prelude::*;

use crate::config::keys::RAPIER_RENDER;
use crate::player::PlayerCamera;
use crate::player::PlayerPlugin;
use crate::world::WorldChunksPlugin;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }),
        );
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin { enabled: false, ..Default::default() });
        app.add_plugins(WindowManagerPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(WorldChunksPlugin);
        app.add_systems(Update, debug_render_toggle);
        app.add_systems(Update, debug_camera_fov);
    }
}

fn debug_render_toggle(mut render: ResMut<DebugRenderContext>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(RAPIER_RENDER) {
        render.enabled = !render.enabled;
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
