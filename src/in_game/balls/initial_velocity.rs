use avian2d::prelude::ExternalImpulse;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct InitialVelocity(pub Vec2);

pub fn observe_initial_velocity(
    trigger: Trigger<OnAdd, InitialVelocity>,
    initial_velocity: Query<&InitialVelocity>,
    mut commands: Commands,
) {
    let initial_velocity = initial_velocity.get(trigger.target()).unwrap();
    
    commands
        .entity(trigger.target())
        .insert(ExternalImpulse::new(initial_velocity.0));
}
