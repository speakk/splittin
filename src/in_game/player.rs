use crate::in_game::ammo_ball::{AmmoBall, InitialVelocity};
use crate::in_game::input::{PlayerInputContext, Shoot};
use bevy::prelude::*;
use bevy_enhanced_input::events::Started;
use bevy_enhanced_input::prelude::Actions;
use std::f32::consts::PI;
use bevy::color::Color::Srgba;
use bevy::color::palettes::basic::RED;

#[derive(Component)]
pub struct Player;

pub(super) fn player_plugin(app: &mut App) {
    app.add_observer(observe_add_player);
    app.add_observer(react_to_shoot);
}

const GUN_LENGTH: f32 = 100.0;

fn observe_add_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let mut gizmo = GizmoAsset::default();
    gizmo.line_2d(Vec2::ZERO, Vec2::new(0.0, -GUN_LENGTH), Color::srgb(0.8, 0.6, 0.4));

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
    let bullet_speed = 10000.0;
    let initial_velocity = Vec2::from_angle(rotation) * bullet_speed;

    commands.spawn((
        AmmoBall,
        InitialVelocity(initial_velocity),
        Transform::from_translation(position + (Vec2::from_angle(rotation) * GUN_LENGTH).extend(0.0)),
    ));
}
