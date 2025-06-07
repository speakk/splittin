use bevy::prelude::*;
use avian2d::prelude::*;

pub(in crate::in_game) fn audio_plugin(app: &mut App) {
    app.add_systems(Update, play_collision_sound);
}

fn play_collision_sound(
    mut collision_events: EventReader<CollisionStarted>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for CollisionStarted(_, _) in collision_events.read() {
        // Spawn a new entity with an AudioPlayer that will play once and then despawn
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/ball_hit.flac")),
            PlaybackSettings::DESPAWN,
        ));
    }
} 