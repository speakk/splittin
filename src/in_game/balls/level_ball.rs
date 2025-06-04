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

const BALL_RADIUS: f32 = 30.0;

fn observe_level_ball_add(
    trigger: Trigger<OnAdd, LevelBall>,
    level_ball: Query<&LevelBall>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let level_ball = level_ball.get(trigger.target()).unwrap();

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::splat(BALL_RADIUS)),
            ..Default::default()
        },
        Collider::circle(BALL_RADIUS / 2.0 as Scalar),
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
        // First, try to find which entity is the static level ball
        let static_level_ball = if let Ok(ball1) = level_ball.get(*entity1) {
            if ball1.static_body {
                Some(*entity1)
            } else {
                None
            }
        } else {
            None
        }.or_else(|| {
            if let Ok(ball2) = level_ball.get(*entity2) {
                if ball2.static_body {
                    Some(*entity2)
                } else {
                    None
                }
            } else {
                None
            }
        });

        // If we didn't find a static level ball, skip this collision
        let Some(static_level_ball) = static_level_ball else {
            continue;
        };

        // The colliding entity is the other one
        let colliding_entity = if static_level_ball == *entity1 {
            *entity2
        } else {
            *entity1
        };

        // Verify the colliding entity is either an ammo ball or a non-static level ball
        if !ammo_ball.contains(colliding_entity) && 
           !level_ball.get(colliding_entity).map_or(false, |ball| !ball.static_body) {
            continue;
        }

        // Get the contact pair for these entities
        let contact_pair = collisions.collisions_with(static_level_ball)
            .find(|pair| pair.collider1 == colliding_entity || pair.collider2 == colliding_entity);
        
        let Some(contact_pair) = contact_pair else {
            continue;
        };

        let position_1 = transforms.get(colliding_entity).unwrap().translation.truncate();
        let position_2 = transforms.get(static_level_ball).unwrap().translation.truncate();
        
        // Get the collision direction from colliding_entity to static_level_ball
        let collision_dir = (position_2 - position_1).normalize();
        
        // Rotate collision direction 90 degrees clockwise and counter-clockwise
        let angle_away_1 = Vec2::new(-collision_dir.y, collision_dir.x); // 90 degrees clockwise
        let angle_away_2 = Vec2::new(collision_dir.y, -collision_dir.x); // 90 degrees counter-clockwise

        commands.entity(static_level_ball).try_despawn();
        commands.entity(colliding_entity).try_despawn();

        println!("collision direction: {:?}", collision_dir);
        println!("split directions: {:?} and {:?}", angle_away_1, angle_away_2);

        let transform = transforms.get(static_level_ball).unwrap();
        let translation = transform.translation;

        // Calculate speed based on contact information
        let base_speed = 3000.0;
        let speed = if let Some(manifold) = contact_pair.manifolds.first() {
            if let Some(contact) = manifold.points.first() {
                // Use penetration to scale the base speed
                // The deeper the penetration, the faster the split
                let penetration_factor = (-contact.penetration / BALL_RADIUS).clamp(0.2, 1.5);
                
                // Also factor in the normal impulse, but scale it down significantly
                // This helps maintain some influence from the collision force
                let impulse_factor = (contact.normal_impulse / 1000.0).clamp(0.5, 2.0);
                
                println!("penetration: {}, penetration_factor: {}", contact.penetration, penetration_factor);
                println!("normal_impulse: {}, impulse_factor: {}", contact.normal_impulse, impulse_factor);
                
                // Combine both factors
                base_speed * penetration_factor * impulse_factor
            } else {
                base_speed
            }
        } else {
            base_speed
        };
        
        let gap_between_balls = 3.0;
        
        println!("final speed: {}", speed);
        
        commands.spawn((
            LevelBall {
                static_body: false
            },
            Transform::from_translation(translation + angle_away_1.extend(0.0) * (BALL_RADIUS / 2.0 + gap_between_balls)),
            InitialVelocity(angle_away_1 * speed),
        ));
        
        commands.spawn((
            LevelBall {
                static_body: false
            },
            Transform::from_translation(translation + angle_away_2.extend(0.0) * (BALL_RADIUS / 2.0 + gap_between_balls)),
            InitialVelocity(angle_away_2 * speed),
        ));
    }
}