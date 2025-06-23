use bevy::prelude::*;

/// Collection of camera logic and configuration.
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_camera);
    }
}

impl CameraPlugin {
    /// Initializes the camera.
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }
}
