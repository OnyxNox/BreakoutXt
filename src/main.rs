mod components;
mod constants;
mod plugins;

use bevy::prelude::*;

use crate::plugins::GamePlugins;

/// Window's clear color.
const CLEAR_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);

/// Game application entry point.
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugins))
        .insert_resource(ClearColor(CLEAR_COLOR))
        .run();
}
