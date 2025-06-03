use crate::in_game::input::PlayerInputContext;
use bevy::prelude::*;
use avian2d::{math::*, prelude::*};

use bevy_enhanced_input::actions::Actions;

#[derive(Component)]
pub struct Ball;

pub(super) fn ball_plugin(app: &mut App) {
    app.add_observer(observe_ball_add);
}

fn observe_ball_add(
    trigger: Trigger<OnAdd, Ball>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_radius = 30.0;

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::splat(ball_radius)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::circle(ball_radius as Scalar),
        Actions::<PlayerInputContext>::default(),
    ));
}
