use std::f32::consts::PI;
use crate::in_game::player::Player;
use bevy::prelude::*;
use bevy::prelude::KeyCode::Space;
use bevy_enhanced_input::prelude::*;

pub(super) fn input_plugin(app: &mut App) {
    app.add_input_context::<PlayerInputContext>();
    app.add_observer(binding);
    app.add_observer(apply_movement);
}

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct Shoot;

#[derive(InputContext)]
pub struct PlayerInputContext;

const PLAYER_SPEED: f32 = 5.0;
const PLAYER_ROTATION_SPEED: f32 = 0.02;

fn binding(
    trigger: Trigger<Binding<PlayerInputContext>>,
    mut players: Query<&mut Actions<PlayerInputContext>>,
) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), Axial::left_stick()))
        .with_modifiers((
            DeadZone::default(),
            SmoothNudge::default(),
            Scale::splat(PLAYER_SPEED),
        ));
    
    actions
        .bind::<Shoot>()
        .to(Space).to(MouseButton::Left);
}

fn apply_movement(trigger: Trigger<Fired<Move>>, mut players: Query<&mut Transform, With<Player>>) {
    let mut transform = players.get_mut(trigger.target()).unwrap();
    transform.translation += trigger.value.extend(0.0).with_y(0.0);
    
    let mut current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
    current_rotation += trigger.value.y * PLAYER_ROTATION_SPEED;
    current_rotation = current_rotation.clamp(-PI/2.0, PI/2.0);
    transform.rotation = Quat::from_rotation_z(current_rotation);
}