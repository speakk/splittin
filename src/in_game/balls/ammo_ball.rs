use crate::in_game::input::PlayerInputContext;
use bevy::prelude::*;
use avian2d::{math::*, prelude::*};
use avian2d::dynamics::integrator::IntegrationSet::Velocity;
use bevy::color::palettes::basic::GREEN;
use bevy_enhanced_input::actions::Actions;

#[derive(Component)]
pub struct AmmoBall;

pub(in crate::in_game) fn ammo_ball_plugin(app: &mut App) {
    app.add_observer(observe_ammo_ball_add);
}

fn observe_ammo_ball_add(
    trigger: Trigger<OnAdd, AmmoBall>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_radius = 30.0;

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::splat(ball_radius)),
            color: GREEN.into(),
            ..Default::default()
        },
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        Mass(32.0),
        Collider::circle(ball_radius / 2.0 as Scalar),
    ));
}
