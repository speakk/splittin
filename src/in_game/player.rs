use crate::in_game::input::PlayerInputContext;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Actions;

#[derive(Component)]
pub struct Player;

pub(super) fn player_plugin(app: &mut App) {
    app.add_observer(observe_add_player);
}

fn observe_add_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("player_ball.png"),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Actions::<PlayerInputContext>::default(),
    ));
}
