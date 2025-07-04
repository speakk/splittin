use bevy::prelude::*;

pub(super) fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera2d,
            Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 1200.0 },
                ..OrthographicProjection::default_2d()
            }),
        ));
    });
}
