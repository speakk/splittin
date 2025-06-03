mod in_game;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::in_game::in_game_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins(in_game_plugin)
        .run();
}
