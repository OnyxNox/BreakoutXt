mod components;
mod constants;
mod enums;
mod events;
mod plugins;
mod resources;
mod states;
mod utility;

use bevy::prelude::*;

use crate::{plugins::GamePlugins, states::GameState};

const CLEAR_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);

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
        .insert_resource(ClearColor(CLEAR_COLOR))
        .init_state::<GameState>()
        .run();
}
