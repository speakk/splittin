use crate::in_game::balls::ammo_ball::AmmoBall;
use crate::in_game::input::{PlayerInputContext, Shoot};
use avian2d::prelude::ExternalImpulse;
use bevy::prelude::*;
use bevy_enhanced_input::events::Started;
use bevy_enhanced_input::prelude::Actions;
use std::f32::consts::PI;
use crate::in_game::balls::initial_velocity::InitialVelocity;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Player;

pub(super) fn player_plugin(app: &mut App) {
    app.add_observer(observe_add_player);
    app.add_observer(react_to_shoot)
        .add_systems(Update, rotate_player_to_mouse);
}

const GUN_LENGTH: f32 = 100.0;

fn observe_add_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let mut gizmo = GizmoAsset::default();
    gizmo.line_2d(
        Vec2::ZERO,
        Vec2::new(0.0, -GUN_LENGTH),
        Color::srgb(0.8, 0.6, 0.4),
    );

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("player_ball.png"),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Actions::<PlayerInputContext>::default(),
        children![Gizmo {
            handle: gizmo_assets.add(gizmo),
            line_config: GizmoLineConfig {
                width: 20.0,
                ..Default::default()
            },
            ..Default::default()
        }],
    ));
}

fn react_to_shoot(
    trigger: Trigger<Started<Shoot>>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    let transform = transforms.get(trigger.target()).unwrap();
    let position = transform.translation;
    let rotation = transform.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
    let bullet_speed = 13_000.0;
    let initial_velocity = Vec2::from_angle(rotation) * bullet_speed;

    commands.spawn((
        AmmoBall,
        InitialVelocity(initial_velocity),
        Transform::from_translation(
            position + (Vec2::from_angle(rotation) * GUN_LENGTH).extend(0.0),
        ),
    ));
}

fn rotate_player_to_mouse(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let window = match window_query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    // Convert cursor position to world coordinates
    let world_position = match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    for mut transform in player_query.iter_mut() {
        let player_pos = transform.translation.truncate();
        let direction = world_position - player_pos;
        let angle = direction.y.atan2(direction.x) + PI / 2.0;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}
