use bevy::pbr::NotShadowCaster;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;

pub struct SkyBoxPlugin;

impl Plugin for SkyBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, skybox_setup);
        app.add_systems(Update, skybox_follow);
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
        .insert(Mesh3d(meshes.add(Cuboid::new(1500., 1500., 1500.))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("sky_side.png")),
            emissive: Color::srgb(100., 100., 100.).into(),
            cull_mode: None,
            unlit: true,
            ..Default::default()
        })))
        .insert(NotShadowCaster)
        .insert(NotShadowReceiver)
        .insert(Transform::default());
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
