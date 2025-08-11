use bevy::pbr::NotShadowCaster;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;

use crate::config::aesthetics::AMBIENT_COLOR;
use crate::config::aesthetics::AMBIENT_STRENGTH;
use crate::config::aesthetics::SKYBOX_SIZE;
use crate::config::aesthetics::SUN_COLOR;
use crate::config::aesthetics::SUN_STRENGTH;

pub struct SkyBoxPlugin;

impl Plugin for SkyBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, skybox_setup);
        app.add_systems(Update, skybox_follow);
        app.add_systems(Update, sun_rotate);
        app.add_systems(Update, sun_attenuate);
    }
}

#[derive(Component)]
struct SkyBox;

#[derive(Component)]
pub struct SkyBoxAttachment;

fn skybox_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SkyBox)
        .insert(Mesh3d(meshes.add(Cuboid::new(SKYBOX_SIZE, SKYBOX_SIZE, SKYBOX_SIZE))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("sky_side.png")),
            cull_mode: None,
            unlit: true,
            ..Default::default()
        })))
        .insert(NotShadowCaster)
        .insert(NotShadowReceiver)
        .insert(Transform::default());

    commands
        .spawn(DirectionalLight {
            color: SUN_COLOR,
            shadows_enabled: true,
            illuminance: SUN_STRENGTH,
            ..Default::default()
        })
        .insert(Transform::default().looking_at(Vec3::new(0.5, -2., 1.), Vec3::Y));

    commands.insert_resource(AmbientLight {
        color: AMBIENT_COLOR,
        brightness: AMBIENT_STRENGTH,
        ..Default::default()
    });
}

fn skybox_follow(
    mut skybox: Query<&mut Transform, With<SkyBox>>,
    leader: Query<&GlobalTransform, (With<SkyBoxAttachment>, Without<SkyBox>)>,
) {
    if let Ok(leader_transform) = leader.single_inner() {
        for mut skybox_transform in &mut skybox {
            skybox_transform.translation = leader_transform.translation();
        }
    }
}

fn sun_rotate() {}

fn sun_attenuate() {}
