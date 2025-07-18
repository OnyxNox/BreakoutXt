mod button_plugin;
mod hud_plugin;
mod main_menu_plugin;

use bevy::prelude::*;

pub struct UserInterfacePlugins;
impl Plugin for UserInterfacePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            self::button_plugin::ButtonPlugin,
            self::hud_plugin::HudPlugin,
            self::main_menu_plugin::MainMenuPlugin,
        ));
    }
}
