mod camera_plugin;
mod hud_plugin;
mod paddle_plugin;
mod wall_plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// Collection of plugins that make up the game.
pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(camera_plugin::CameraPlugin)
            .add(hud_plugin::HudPlugin)
            .add(paddle_plugin::PaddlePlugin)
            .add(wall_plugin::WallPlugin)
    }
}
