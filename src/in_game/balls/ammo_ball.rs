use crate::in_game::balls::level_ball::PreviousVelocity;
use crate::in_game::input::PlayerInputContext;
use bevy::prelude::*;
use avian2d::{math::*, prelude::*};
use avian2d::dynamics::integrator::IntegrationSet::Velocity;
use bevy::color::palettes::basic::GREEN;
use bevy_enhanced_input::actions::Actions;

#[derive(Component)]
pub struct AmmoBall;

// Tracks which ammo ball caused this chain reaction
#[derive(Component, Clone)]
pub struct SplitChain {
    pub ammo_id: u32,
}

// Resource to generate unique IDs for ammo balls
#[derive(Resource)]
struct NextAmmoId(u32);

impl Default for NextAmmoId {
    fn default() -> Self {
        Self(1) // Start from 1, 0 could be used to mean "no chain"
    }
}

pub(in crate::in_game) fn ammo_ball_plugin(app: &mut App) {
    app.init_resource::<NextAmmoId>()
        .add_observer(observe_ammo_ball_add);
}

fn observe_ammo_ball_add(
    trigger: Trigger<OnAdd, AmmoBall>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_id: ResMut<NextAmmoId>,
) {
    let ball_radius = 30.0;

    let mut entity_commands = commands.entity(trigger.target());
    entity_commands.insert((
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
        SplitChain {
            ammo_id: next_id.0,
        },
        PreviousVelocity(Vec2::ZERO),
    ));
    
    next_id.0 += 1;
}
