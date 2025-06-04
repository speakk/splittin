use crate::in_game::balls::initial_velocity::observe_initial_velocity;
use bevy::prelude::*;
use crate::in_game::balls::ammo_ball::ammo_ball_plugin;
use crate::in_game::balls::level_ball::level_ball_plugin;

pub mod ammo_ball;
pub mod initial_velocity;
pub mod level_ball;

pub(super) fn balls_plugin(app: &mut App) {
    app.add_observer(observe_initial_velocity);
    app.add_plugins((level_ball_plugin, ammo_ball_plugin));
}
