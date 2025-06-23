mod audio_plugin;
mod ball_plugin;
mod brick_plugin;
mod camera_plugin;
mod game_plugin;
mod hud_plugin;
mod main_menu_plugin;
mod paddle_plugin;
mod physics_plugin;
mod splash_plugin;
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
            .add(self::hud_plugin::HudPlugin)
            .add(self::main_menu_plugin::MainMenuPlugin)
            .add(self::paddle_plugin::PaddlePlugin)
            .add(self::physics_plugin::PhysicsPlugin)
            .add(self::splash_plugin::SplashPlugin)
            .add(self::wall_plugin::WallPlugin)
    }
}
