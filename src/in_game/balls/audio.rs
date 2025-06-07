use bevy::prelude::*;
use avian2d::prelude::*;
use rand::Rng;
use crate::in_game::balls::level_ball::LevelBall;

// Configuration for the collision sound
#[derive(Resource)]
pub struct CollisionSoundConfig {
    /// Base playback speed (1.0 is normal)
    pub base_speed: f32,
    /// Maximum random variation in playback speed (e.g. 0.2 means ±20%)
    pub speed_variation: f32,
    /// How much to increase pitch per dynamic ball
    pub pitch_per_ball: f32,
    /// Maximum pitch multiplier regardless of ball count
    pub max_pitch: f32,
}

impl Default for CollisionSoundConfig {
    fn default() -> Self {
        Self {
            base_speed: 1.0,
            speed_variation: 0.1, // 20% variation by default
            pitch_per_ball: 0.06, // 10% increase per ball
            max_pitch: 2.5, // Maximum 2.5x pitch
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
    level_balls: Query<&LevelBall>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();
    
    // Count dynamic balls
    let dynamic_ball_count = level_balls
        .iter()
        .filter(|ball| !ball.static_body)
        .count();
    
    // Calculate pitch multiplier based on dynamic ball count
    let dynamic_pitch = (1.0 + (dynamic_ball_count as f32 * config.pitch_per_ball))
        .clamp(1.0, config.max_pitch);
    
    for CollisionStarted(_, _) in collision_events.read() {
        // Calculate random speed variation
        let speed_variation = rng.random_range(-config.speed_variation..=config.speed_variation);
        let final_speed = config.base_speed * dynamic_pitch + (config.base_speed * speed_variation);
        
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