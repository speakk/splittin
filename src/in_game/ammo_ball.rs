use crate::in_game::input::PlayerInputContext;
use bevy::prelude::*;
use avian2d::{math::*, prelude::*};
use avian2d::dynamics::integrator::IntegrationSet::Velocity;
use bevy::color::palettes::basic::GREEN;
use bevy_enhanced_input::actions::Actions;

#[derive(Component)]
#[require(InitialVelocity = InitialVelocity(Vec2::ZERO))]
pub struct AmmoBall;

#[derive(Component)]
pub struct InitialVelocity(pub Vec2);

pub(super) fn ammo_ball_plugin(app: &mut App) {
    app.add_observer(observe_ammo_ball_add);
}

fn observe_ammo_ball_add(
    trigger: Trigger<OnAdd, AmmoBall>,
    initial_velocity: Query<&InitialVelocity>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_radius = 30.0;
    let initial_velocity = initial_velocity.get(trigger.target()).unwrap();

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::splat(ball_radius)),
            color: GREEN.into(),
            ..Default::default()
        },
        RigidBody::Dynamic,
        ExternalImpulse::new(initial_velocity.0 * 100.0),
        Collider::circle(ball_radius as Scalar),
    ));
}
