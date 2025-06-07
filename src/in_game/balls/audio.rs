use bevy::prelude::*;
use avian2d::prelude::*;
use rand::Rng;
use crate::in_game::balls::level_ball::{LevelBall, SplitChain};

// Configuration for the collision sound
#[derive(Resource)]
pub struct CollisionSoundConfig {
    /// Base playback speed (1.0 is normal)
    pub base_speed: f32,
    /// Maximum random variation in playback speed (e.g. 0.2 means ±20%)
    pub speed_variation: f32,
    /// How much to increase pitch per ball in chain
    pub pitch_per_ball: f32,
    /// Maximum pitch multiplier regardless of ball count
    pub max_pitch: f32,
    /// Maximum number of simultaneous collision sounds
    pub max_simultaneous_sounds: usize,
}

impl Default for CollisionSoundConfig {
    fn default() -> Self {
        Self {
            base_speed: 1.0,
            speed_variation: 0.1,
            pitch_per_ball: 0.06,
            max_pitch: 2.5,
            max_simultaneous_sounds: 8,
        }
    }
}

#[derive(Resource, Default)]
struct ActiveSoundCount(usize);

pub(in crate::in_game) fn audio_plugin(app: &mut App) {
    app.init_resource::<CollisionSoundConfig>()
        .init_resource::<ActiveSoundCount>()
        .add_systems(Update, (play_collision_sound, cleanup_finished_sounds));
}

fn cleanup_finished_sounds(
    mut commands: Commands,
    sinks: Query<(Entity, &AudioSink)>,
    mut active_count: ResMut<ActiveSoundCount>,
) {
    for (entity, sink) in sinks.iter() {
        if sink.empty() {
            commands.entity(entity).despawn();
            active_count.0 = active_count.0.saturating_sub(1);
        }
    }
}

fn play_collision_sound(
    mut collision_events: EventReader<CollisionStarted>,
    asset_server: Res<AssetServer>,
    config: Res<CollisionSoundConfig>,
    mut commands: Commands,
    split_chains: Query<(&LevelBall, &SplitChain)>,
    mut active_count: ResMut<ActiveSoundCount>,
) {
    let mut rng = rand::rng();

    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // Skip if we've reached the maximum number of simultaneous sounds
        if active_count.0 >= config.max_simultaneous_sounds {
            continue;
        }

        // Try to get the chain ID from either entity in the collision
        let chain_id = if let Ok((_, chain)) = split_chains.get(*entity1) {
            Some(chain.ammo_id)
        } else if let Ok((_, chain)) = split_chains.get(*entity2) {
            Some(chain.ammo_id)
        } else {
            None
        };

        if let Some(chain_id) = chain_id {
            // Count how many dynamic balls are in this chain
            let chain_count = split_chains
                .iter()
                .filter(|(ball, chain)| !ball.static_body && chain.ammo_id == chain_id)
                .count();

            // Calculate pitch based on chain count
            let base_pitch = 1.0 + (chain_count as f32 * config.pitch_per_ball);
            let pitch = base_pitch.min(config.max_pitch);

            // Add small random variation to avoid identical sounds
            let variation = rng.random_range(-config.speed_variation..=config.speed_variation);
            let final_pitch = pitch * (1.0 + variation);

            // Spawn a new entity with an AudioPlayer that will play once
            commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/ball_hit.flac")),
                PlaybackSettings {
                    speed: final_pitch,
                    ..PlaybackSettings::ONCE // Just play once, we'll handle despawning ourselves
                },
            ));

            active_count.0 += 1;
        }
    }
} 