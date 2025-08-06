use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, player_move);
        app.add_systems(Update, player_look);
        app.add_systems(Update, player_interact);
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
    look_speed: f32,
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Player { speed: 10., look_speed: 0.001 })
        .insert(Camera3d::default())
        .insert(Mesh3d(meshes.add(Capsule3d::new(0.5, 1.5))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0., 1., 1.)))))
        .insert(KinematicCharacterController { ..Default::default() })
        .insert(Collider::capsule_y(0.75, 0.5))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform::from_xyz(10., 40., 10.));
}

fn player_move(
    mut query: Query<(&mut KinematicCharacterController, &Transform, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut controller, transform, settings)) = query.single_mut()
    else {
        panic!("should be a player initialized");
    };
    let dt = time.delta_secs();

    let [f, r, u] = [
        transform.forward().with_y(0.).normalize(),
        transform.right().normalize(),
        Vec3::Y,
    ];

    let mut movement = Vec3::ZERO;
    for key in keys.get_pressed() {
        match key {
            | KeyCode::KeyW => movement += f,
            | KeyCode::KeyS => movement -= f,
            | KeyCode::KeyA => movement -= r,
            | KeyCode::KeyD => movement += r,
            | KeyCode::Space => movement += u,
            | _ => {}
        }
    }

    let desired = movement.normalize_or_zero() * settings.speed * dt;

    controller.translation = Some(desired);
}

fn player_look(
    mut query: Query<(&mut Transform, &Player)>,
    mouse: Res<AccumulatedMouseMotion>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.single_inner()
    else {
        panic!("should have a window initialized");
    };

    for (mut transform, player) in &mut query {
        let delta = mouse.delta;
        let (dpitch, dyaw) = (
            -delta.y * player.look_speed * window.scale_factor(),
            -delta.x * player.look_speed * window.scale_factor(),
        );

        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw + dyaw,
            (pitch + dpitch).to_degrees().clamp(-89., 89.).to_radians(),
            0.,
        );
    }
}

fn player_interact() {}
