use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use avian2d::prelude::*;

pub(in crate::in_game) fn particles_plugin(app: &mut App) {
    app.add_plugins(HanabiPlugin)
        .add_systems(Startup, setup_particle_effects)
        .add_systems(Update, (spawn_collision_particles, cleanup_finished_effects));
}

fn setup_particle_effects(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Create a new effect asset
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.3, 1.0));  // Bright yellow
    gradient.add_key(0.5, Vec4::new(1.0, 0.4, 0.0, 0.8));  // Orange
    gradient.add_key(1.0, Vec4::new(0.5, 0.0, 0.0, 0.0));  // Fade to transparent red

    // Create a new expression module
    let mut module = Module::default();

    // Initialize position at the collision point (will be set when spawning)
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(10.0),
        dimension: ShapeDimension::Surface,
    };

    // Initialize velocity in a sphere pattern for a nice explosion effect
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(50.0),
    };

    // Set particle lifetime
    let lifetime = module.lit(0.5); // half second lifetime
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Set particle size
    let size = module.lit(5.0);
    let init_size = SetAttributeModifier::new(Attribute::SIZE, size);

    let effect = EffectAsset::new(
        32,  // Maximum number of particles
        SpawnerSettings::once(20.0.into()),  // Spawn 20 particles immediately
        module,
    )
    .with_name("collision_effect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .init(init_size)
    .render(ColorOverLifetimeModifier {
        gradient,
        blend: ColorBlendMode::Overwrite,
        mask: ColorBlendMask::RGBA,
    });

    // Spawn the effect asset
    commands.insert_resource(CollisionEffectTemplate(effects.add(effect)));
}

#[derive(Resource)]
struct CollisionEffectTemplate(Handle<EffectAsset>);

fn spawn_collision_particles(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    effect_template: Res<CollisionEffectTemplate>,
    transforms: Query<&Transform>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // Get the collision position (average of both entities' positions)
        if let (Ok(transform1), Ok(transform2)) = (transforms.get(*entity1), transforms.get(*entity2)) {
            let collision_pos = (transform1.translation + transform2.translation) / 2.0;
            
            // Spawn a new particle effect at the collision point
            commands.spawn((
                ParticleEffect::new(effect_template.0.clone()),
                Transform::from_translation(collision_pos),
                // Add a component to track when this effect should be cleaned up
                CleanupAfter(Timer::from_seconds(0.6, TimerMode::Once)), // slightly longer than particle lifetime
            ));
        }
    }
}

// Component to track when to clean up the effect
#[derive(Component)]
struct CleanupAfter(Timer);

fn cleanup_finished_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CleanupAfter)>,
) {
    for (entity, mut cleanup_timer) in query.iter_mut() {
        cleanup_timer.0.tick(time.delta());
        if cleanup_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
} 