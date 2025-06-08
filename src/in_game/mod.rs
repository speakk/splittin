mod camera;
mod input;
mod player;
mod balls;
mod levels;

use crate::in_game::camera::camera_plugin;
use crate::in_game::input::input_plugin;
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use balls::{ammo_ball, level_ball};
use crate::in_game::balls::balls_plugin;
use crate::in_game::levels::{CurrentLevel, LevelLoadingPlugin};

pub(super) fn in_game_plugin(app: &mut App) {
    app.add_plugins((
        camera_plugin,
        input_plugin,
        player::player_plugin,
        balls_plugin,
        LevelLoadingPlugin,
    ));
    app.add_systems(Startup, start_level);
}

fn start_level(
    mut commands: Commands,
) {
    // Load the first level
    commands.insert_resource(CurrentLevel {
        path: "assets/levels/level_1.tmx".to_string(),
    });
}
