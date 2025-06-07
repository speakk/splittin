mod in_game;

use avian2d::PhysicsPlugins;
use avian2d::prelude::{Gravity, PhysicsDebugPlugin};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::in_game::in_game_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default(),)
        .insert_resource(Gravity(Vec2::NEG_Y * 380.0))
        .add_plugins(in_game_plugin)
        .run();
}
