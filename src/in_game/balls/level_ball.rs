use crate::in_game::balls::ammo_ball::AmmoBall;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use crate::in_game::balls::initial_velocity::InitialVelocity;

#[derive(Component)]
pub struct LevelBall {
    pub static_body: bool,
}

pub(in crate::in_game) fn level_ball_plugin(app: &mut App) {
    app.add_observer(observe_level_ball_add);
    app.add_systems(Update, react_to_ammo_ball_hitting);
}

fn observe_level_ball_add(
    trigger: Trigger<OnAdd, LevelBall>,
    level_ball: Query<&LevelBall>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_radius = 30.0;
    let level_ball = level_ball.get(trigger.target()).unwrap();

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::splat(ball_radius)),
            ..Default::default()
        },
        Collider::circle(ball_radius / 2.0 as Scalar),
        CollisionEventsEnabled,
        Mass(6.0),
        if level_ball.static_body {
            RigidBody::Static
        } else {
            RigidBody::Dynamic
        }
    ));
}

fn react_to_ammo_ball_hitting(
    mut event: EventReader<CollisionStarted>,
    transforms: Query<&Transform>,
    collisions: Collisions,
    level_ball: Query<&LevelBall>,
    ammo_ball: Query<(), With<AmmoBall>>,
    mut commands: Commands,
) {
    for CollisionStarted(entity1, entity2) in event.read() {
        
        
        let ammo_entity: Option<Entity> = if ammo_ball.contains(*entity1) {
            Some(*entity1)
        } else if ammo_ball.contains(*entity2) {
            Some(*entity2)
        } else {
            None
        };

        let level_ball_entity: Option<Entity> = if level_ball.contains(*entity1) {
            Some(*entity1)
        } else if level_ball.contains(*entity2) {
            Some(*entity2)
        } else {
            None
        };

        if let Some(ammo_ball_entity) = ammo_entity {
            if let Some(level_ball_entity) = level_ball_entity {
                let position_1 = transforms.get(ammo_ball_entity).unwrap().translation.truncate();
                let position_2 = transforms.get(level_ball_entity).unwrap().translation.truncate();
                let direction = (position_2 - position_1).normalize();
                let angle_away_1 = Vec2::new(-direction.y, direction.x); // Rotates 90 degrees clockwise
                let angle_away_2 = Vec2::new(direction.y, -direction.x); // Rotates 90 degrees counter-clockwise
                commands.entity(level_ball_entity).despawn();
                commands.entity(ammo_ball_entity).despawn();

                println!("angle away: {:?}", angle_away_1);

                let transform = transforms.get(level_ball_entity).unwrap();
                let translation = transform.translation;

                commands.spawn((
                    LevelBall {
                        static_body: false
                    },
                    Transform::from_translation(translation + angle_away_1.extend(0.0) * 15.0),
                    InitialVelocity(angle_away_1 * 1000.0),
                ));
                
                commands.spawn((
                    LevelBall {
                        static_body: false
                    },
                    Transform::from_translation(translation + angle_away_2.extend(0.0) * 15.0),
                    InitialVelocity(angle_away_2 * 1000.0),
                ));
            }
        }
    }
}