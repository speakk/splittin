mod ball;
mod camera;
mod input;
mod player;

use crate::in_game::camera::camera_plugin;
use crate::in_game::input::input_plugin;
use crate::in_game::player::Player;
use bevy::prelude::*;

pub(super) fn in_game_plugin(app: &mut App) {
    app.add_plugins((
        camera_plugin,
        input_plugin,
        player::player_plugin,
        ball::ball_plugin,
    ));
    app.add_systems(Startup, start_level);
}

fn start_level(mut commands: Commands) {
    commands.spawn((Player, Transform::from_xyz(0.0, 200.0, 0.0)));

    let rows = 5;
    let balls_per_row = 10;
    let ball_spacing = 100.0;
    let row_spacing = 100.0;
    
    for row in 0..rows {
        for i in 0..balls_per_row {
            let x = (i as f32 - (balls_per_row - 1) as f32 / 2.0) * ball_spacing;
            let y = row as f32 * row_spacing;
            commands.spawn((ball::Ball, Transform::from_xyz(x, -y, 0.0)));
        }
    }
}
