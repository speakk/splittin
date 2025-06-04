use std::f32::consts::PI;
use crate::in_game::ammo_ball::{AmmoBall, InitialVelocity};
use crate::in_game::input::{PlayerInputContext, Shoot};
use bevy::prelude::*;
use bevy_enhanced_input::events::Started;
use bevy_enhanced_input::prelude::Actions;

#[derive(Component)]
pub struct Player;

pub(super) fn player_plugin(app: &mut App) {
    app.add_observer(observe_add_player);
    app.add_observer(react_to_shoot);
}

fn observe_add_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("player_ball.png"),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Actions::<PlayerInputContext>::default(),
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
        Transform::from_translation(position),
    ));
}
