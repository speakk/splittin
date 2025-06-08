use crate::in_game::balls::ammo_ball::AmmoBall;
use crate::in_game::input::{PlayerInputContext, Shoot, IncreaseForce, DecreaseForce};
use avian2d::prelude::ExternalImpulse;
use bevy::prelude::*;
use bevy_enhanced_input::events::{Started, Fired};
use bevy_enhanced_input::prelude::Actions;
use std::f32::consts::PI;
use crate::in_game::balls::initial_velocity::InitialVelocity;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct GunGizmo;

#[derive(Component)]
struct ForceGizmo;

#[derive(Component)]
pub struct ShootingForce {
    value: f32,
    min: f32,
    max: f32,
    step: f32,
}

impl Default for ShootingForce {
    fn default() -> Self {
        Self {
            value: 13_000.0, // Default bullet speed
            min: 5_000.0,
            max: 100_000.0,
            step: 1_000.0,
        }
    }
}

pub(super) fn player_plugin(app: &mut App) {
    app.add_observer(observe_add_player)
        .add_observer(react_to_shoot)
        .add_observer(react_to_increase_force)
        .add_observer(react_to_decrease_force)
        .add_systems(Update, (rotate_player_to_mouse, update_force_gizmo).chain());
}

const GUN_LENGTH: f32 = 100.0;
const FORCE_GIZMO_WIDTH: f32 = 80.0; // This is now the length of the force indicator
const FORCE_GIZMO_THICKNESS: f32 = 4.0;
const FORCE_OFFSET: f32 = 10.0; // Distance from gun barrel

fn observe_add_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let mut gun_gizmo = GizmoAsset::default();
    gun_gizmo.line_2d(
        Vec2::ZERO,
        Vec2::new(0.0, -GUN_LENGTH),
        Color::srgb(0.8, 0.6, 0.4),
    );

    let mut force_gizmo = GizmoAsset::default();
    force_gizmo.line_2d(
        Vec2::new(FORCE_OFFSET, 0.0), // Start point offset from gun
        Vec2::new(FORCE_OFFSET, -FORCE_GIZMO_WIDTH), // End point parallel to gun
        Color::srgb(0.0, 1.0, 1.0), // Cyan color
    );

    commands.entity(trigger.target()).insert((
        Sprite {
            image: asset_server.load("player_ball.png"),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Actions::<PlayerInputContext>::default(),
        ShootingForce::default(),
        children![
            // Gun gizmo
            (
                Gizmo {
                    handle: gizmo_assets.add(gun_gizmo),
                    line_config: GizmoLineConfig {
                        width: 20.0,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                GunGizmo,
            ),
            // Force indicator gizmo
            (
                Gizmo {
                    handle: gizmo_assets.add(force_gizmo),
                    line_config: GizmoLineConfig {
                        width: FORCE_GIZMO_THICKNESS,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ForceGizmo,
            ),
        ],
    ));
}

fn react_to_shoot(
    trigger: Trigger<Started<Shoot>>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    forces: Query<&ShootingForce>,
) {
    let transform = transforms.get(trigger.target()).unwrap();
    let position = transform.translation;
    let rotation = transform.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
    let force = forces.get(trigger.target()).unwrap();
    let initial_velocity = Vec2::from_angle(rotation) * force.value;

    commands.spawn((
        AmmoBall,
        InitialVelocity(initial_velocity),
        Transform::from_translation(
            position + (Vec2::from_angle(rotation) * GUN_LENGTH).extend(0.0),
        ),
    ));
}

fn rotate_player_to_mouse(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let window = match window_query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    // Convert cursor position to world coordinates
    let world_position = match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    for mut transform in player_query.iter_mut() {
        let player_pos = transform.translation.truncate();
        let direction = world_position - player_pos;
        let angle = direction.y.atan2(direction.x) + PI / 2.0;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn react_to_increase_force(
    trigger: Trigger<Fired<IncreaseForce>>,
    mut forces: Query<&mut ShootingForce>,
) {
    if let Ok(mut force) = forces.get_mut(trigger.target()) {
        force.value = (force.value + force.step).min(force.max);
    }
}

fn react_to_decrease_force(
    trigger: Trigger<Fired<DecreaseForce>>,
    mut forces: Query<&mut ShootingForce>,
) {
    if let Ok(mut force) = forces.get_mut(trigger.target()) {
        force.value = (force.value - force.step).max(force.min);
    }
}

fn update_force_gizmo(
    force_query: Query<&ShootingForce>,
    mut gizmos: Query<(&mut Transform, &Gizmo), With<ForceGizmo>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    for force in force_query.iter() {
        for (mut transform, gizmo) in gizmos.iter_mut() {
            // Calculate force percentage and scale
            let force_percent = (force.value - force.min) / (force.max - force.min);
            // Ensure minimum scale of 0.2 (20%) for visibility
            let scale = 0.2 + (force_percent * 0.8);
            
            // Update transform scale for Y axis only
            transform.scale.y = scale;

            // Update gizmo color based on force
            let mut gizmo_asset = GizmoAsset::default();
            let color = Color::srgb(
                force_percent.min(1.0), // More red as force increases
                (1.0 - force_percent).max(0.0), // More green as force decreases
                1.0, // Keep blue constant for visibility
            );
            gizmo_asset.line_2d(
                Vec2::new(FORCE_OFFSET, 0.0),
                Vec2::new(FORCE_OFFSET, -FORCE_GIZMO_WIDTH),
                color,
            );
            if let Some(asset) = gizmo_assets.get_mut(&gizmo.handle) {
                *asset = gizmo_asset;
            }
        }
    }
}
