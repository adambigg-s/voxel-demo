use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

use crate::block::BlockType;
use crate::block::Voxel;
use crate::block::get_block;
use crate::config::blocks::VOXEL_SIZE;
use crate::config::keys::PLAYER_RESET;
use crate::config::player::BLOCK_REACH;
use crate::skybox::SkyBoxAttachment;
use crate::skybox::SkyBoxPlugin;
use crate::world::BlockBreakEvent;
use crate::world::BlockPlaceEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkyBoxPlugin);
        app.add_systems(Startup, player_setup);
        app.add_systems(Startup, player_block_ui);
        app.add_systems(Update, player_block_ui_update);
        app.add_systems(Update, player_block_select);
        app.add_systems(Update, player_look);
        app.add_systems(Update, player_move);
        app.add_systems(Update, player_interact);
        app.add_systems(Update, player_reset);
    }
}

#[derive(Component)]
struct PlayerUI;

fn player_block_ui(mut commands: Commands) {
    commands
        .spawn(PlayerUI)
        .insert(Text::from("sand"))
        .insert(TextFont { font_size: 15., ..Default::default() })
        .insert(TextColor::BLACK)
        .insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(10.),
            ..Default::default()
        });
}

fn player_block_ui_update(text: Single<&mut Text, With<PlayerUI>>, block: Single<&BlockSelection>) {
    *text.into_inner() = Text::from(String::from(block.block));
}

#[derive(Default, Component)]
struct VerticalVelocity {
    value: f32,
}

#[derive(Component)]
struct BlockSelection {
    block: Voxel,
    index: usize,
}

#[derive(Component)]
struct Player {
    speed: f32,
    look_speed: f32,
    jump_velocity: f32,
    gravity: f32,
}

#[derive(Component)]
pub struct PlayerCamera;

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_collider = commands
        .spawn(Player {
            speed: 5.,
            look_speed: 0.001,
            jump_velocity: 7.5,
            gravity: 25.,
        })
        .insert(Mesh3d(meshes.add(Capsule3d::new(0.3, 1.5))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0., 1., 1.)))))
        .insert(KinematicCharacterController::default())
        .insert(KinematicCharacterControllerOutput::default())
        .insert(Collider::cuboid(0.3, 0.75, 0.3))
        .insert(RigidBody::KinematicPositionBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(VerticalVelocity::default())
        .insert(Transform::from_xyz(10., 40., 10.))
        .id();

    let player_camera = commands
        .spawn(PlayerCamera)
        .insert(Camera3d::default())
        .insert(Camera { is_active: false, ..Default::default() })
        .insert(Msaa::Off)
        .insert(ScreenSpaceAmbientOcclusion {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low,
            ..Default::default()
        })
        .insert(SkyBoxAttachment)
        .insert(Transform::from_xyz(0., 0.75, 0.))
        .id();

    commands
        .spawn(Text::new("+"))
        .insert(TextColor::BLACK)
        .insert(TextFont { font_size: 35., ..Default::default() })
        .insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Vh(50.),
            left: Val::Vw(50.),
            ..Default::default()
        });

    commands.entity(player_collider).add_child(player_camera);

    commands.spawn(BlockSelection { block: Voxel::Full(BlockType::Wood), index: 0 });
}

fn player_look(
    mut cam_query: Single<&mut Transform, With<PlayerCamera>>,
    player_query: Single<&Player>,
    mouse: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let [dyaw, dpitch] = [
        -mouse.delta.x * player_query.look_speed * window.scale_factor(),
        -mouse.delta.y * player_query.look_speed * window.scale_factor(),
    ];
    let (yaw, pitch, ..) = cam_query.rotation.to_euler(EulerRot::YXZ);

    cam_query.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw + dyaw,
        (pitch + dpitch).to_degrees().clamp(-89., 89.).to_radians(),
        0.,
    );
}

fn player_move(
    player_query: Single<(
        &mut KinematicCharacterController,
        &mut VerticalVelocity,
        &KinematicCharacterControllerOutput,
        &Player,
    )>,
    cam_query: Single<&Transform, With<PlayerCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut controller, mut vertical_velocity, control_output, player) = player_query.into_inner();
    let dt = time.delta_secs();
    let (front, right) = (cam_query.forward().with_y(0.).normalize(), cam_query.right().normalize());

    let mut movement = Vec3::ZERO;
    for key in keys.get_pressed() {
        match key {
            | KeyCode::KeyW => movement += front,
            | KeyCode::KeyS => movement -= front,
            | KeyCode::KeyD => movement += right,
            | KeyCode::KeyA => movement -= right,
            | _ => {}
        }
    }
    let horizontal = movement.normalize_or_zero() * player.speed * dt;

    vertical_velocity.value -= player.gravity * dt;
    if control_output.grounded && vertical_velocity.value.is_sign_negative() {
        vertical_velocity.value = 0.;
    }
    if keys.pressed(KeyCode::Space) && control_output.grounded && vertical_velocity.value < 0.5 {
        vertical_velocity.value = player.jump_velocity;
    }

    let total_movemnt = horizontal + Vec3::new(0., vertical_velocity.value * dt, 0.);

    controller.translation = Some(total_movemnt);
}

fn player_interact(
    mut break_events: EventWriter<BlockBreakEvent>,
    mut place_events: EventWriter<BlockPlaceEvent>,
    player_transform: Single<&GlobalTransform, With<PlayerCamera>>,
    player_collider: Single<Entity, With<Player>>,
    player_block: Single<&BlockSelection>,
    context: ReadRapierContext,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(context) = context.single()
    else {
        error!("failed to get rapier context");
        return;
    };

    if let Some(ray_hit) = context.cast_ray_and_get_normal(
        player_transform.translation(),
        player_transform.forward().as_vec3(),
        BLOCK_REACH,
        true,
        QueryFilter::new().exclude_collider(player_collider.into_inner()),
    ) {
        let (.., hit) = ray_hit;

        let break_pos = (hit.point - hit.normal * VOXEL_SIZE / 100.).as_ivec3();
        let place_pos = (hit.point + hit.normal * VOXEL_SIZE / 100.).as_ivec3();
        if mouse.just_pressed(MouseButton::Left) {
            break_events.write(BlockBreakEvent { position: break_pos });
        }
        if mouse.just_pressed(MouseButton::Right) {
            place_events.write(BlockPlaceEvent { position: place_pos, species: player_block.block });
        }
    }
}

fn player_block_select(mut block: Single<&mut BlockSelection>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        block.index = block.index.wrapping_add(1);
        block.block = get_block(block.index);
    }
    if keys.just_pressed(KeyCode::KeyF) {
        block.index = block.index.wrapping_sub(1);
        block.block = get_block(block.index);
    }
}

fn player_reset(mut query: Query<&mut Transform, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(PLAYER_RESET) {
        for mut transform in &mut query {
            *transform = transform.with_translation(Vec3::new(3., 20., 3.));
        }
    }
}
