use bevy::prelude::*;
use avian2d::prelude::*;
use rand::Rng;

// Configuration for the collision sound
#[derive(Resource)]
pub struct CollisionSoundConfig {
    /// Base playback speed (1.0 is normal)
    pub base_speed: f32,
    /// Maximum random variation in playback speed (e.g. 0.2 means ±20%)
    pub speed_variation: f32,
}

impl Default for CollisionSoundConfig {
    fn default() -> Self {
        Self {
            base_speed: 1.0,
            speed_variation: 0.1, // 20% variation by default
        }
    }
}

pub(in crate::in_game) fn audio_plugin(app: &mut App) {
    app.init_resource::<CollisionSoundConfig>()
        .add_systems(Update, play_collision_sound);
}

fn play_collision_sound(
    mut collision_events: EventReader<CollisionStarted>,
    asset_server: Res<AssetServer>,
    config: Res<CollisionSoundConfig>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();
    
    for CollisionStarted(_, _) in collision_events.read() {
        // Calculate random speed variation
        let speed_variation = rng.random_range(-config.speed_variation..=config.speed_variation);
        let final_speed = config.base_speed + (config.base_speed * speed_variation);
        
        // Create playback settings with our custom speed
        let settings = PlaybackSettings {
            speed: final_speed,
            ..PlaybackSettings::DESPAWN
        };

        // Spawn a new entity with an AudioPlayer that will play once and then despawn
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/ball_hit.flac")),
            settings,
        ));
    }
} 