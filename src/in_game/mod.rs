mod camera;
mod input;
mod player;
mod balls;

use crate::in_game::camera::camera_plugin;
use crate::in_game::input::input_plugin;
use balls::level_ball::LevelBall;
use crate::in_game::player::Player;
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use balls::{ammo_ball, level_ball};
use crate::in_game::balls::balls_plugin;

pub(super) fn in_game_plugin(app: &mut App) {
    app.add_plugins((
        camera_plugin,
        input_plugin,
        player::player_plugin,
        balls_plugin,
    ));
    app.add_systems(Startup, start_level);
}

fn start_level(
    mut commands: Commands,
) {
    commands.spawn((Player, Transform::from_xyz(0.0, 750.0, 0.0)));

    let rows = 17;
    let balls_per_row = 40;
    let ball_spacing = 80.0;
    let row_spacing = 80.0;

    for row in 0..rows {
        for i in 0..balls_per_row {
            let x = (i as f32 - (balls_per_row - 1) as f32 / 2.0) * ball_spacing;
            let y = (row as f32 - (rows - 1) as f32 / 2.0) * row_spacing + 100.0;
            commands.spawn((
                LevelBall {
                    static_body: true
                },
                Transform::from_xyz(x, -y, 0.0),
            ));
        }
    }
}
