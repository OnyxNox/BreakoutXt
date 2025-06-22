mod ball_plugin;
mod brick_plugin;
mod camera_plugin;
mod hud_plugin;
mod paddle_plugin;
mod physics_plugin;
mod wall_plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(self::ball_plugin::BallPlugin)
            .add(self::brick_plugin::BrickPlugin)
            .add(self::camera_plugin::CameraPlugin)
            .add(self::hud_plugin::HudPlugin)
            .add(self::paddle_plugin::PaddlePlugin)
            .add(self::physics_plugin::PhysicsPlugin)
            .add(self::wall_plugin::WallPlugin)
    }
}
