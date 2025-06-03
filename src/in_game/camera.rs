use bevy::prelude::*;

pub(super) fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2d::default());
    });
}
