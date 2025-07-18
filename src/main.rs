mod bundles;
mod components;
mod constants;
mod enums;
mod events;
mod plugins;
mod resources;
mod states;
mod traits;
mod utility;

use bevy::prelude::*;

use crate::{plugins::GamePlugins, states::GameState};

/// Color used to clear the screen between frames.
const WINDOW_CLEAR_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);

/// Application entry point.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Breakout XT".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            GamePlugins,
        ))
        .insert_resource(ClearColor(WINDOW_CLEAR_COLOR))
        .init_state::<GameState>()
        .run();
}
