mod audio_plugin;
mod ball_plugin;
mod brick_plugin;
mod camera_plugin;
mod game_plugin;
mod paddle_plugin;
mod physics_plugin;
mod splash_screen_plugin;
mod user_interface_plugins;
mod wall_plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(self::audio_plugin::AudioPlugin)
            .add(self::ball_plugin::BallPlugin)
            .add(self::brick_plugin::BrickPlugin)
            .add(self::camera_plugin::CameraPlugin)
            .add(self::game_plugin::GamePlugin)
            .add(self::paddle_plugin::PaddlePlugin)
            .add(self::physics_plugin::PhysicsPlugin)
            .add(self::splash_screen_plugin::SplashScreenPlugin)
            .add(self::user_interface_plugins::UserInterfacePlugins)
            .add(self::wall_plugin::WallPlugin)
    }
}
