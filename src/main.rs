mod plugins;

use bevy::{DefaultPlugins, app::App, color::Color, render::camera::ClearColor};

use crate::plugins::CameraPlugin;

/// Window's clear color.
const CLEAR_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);

/// Game application entry point.
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin))
        .insert_resource(ClearColor(CLEAR_COLOR))
        .run();
}
