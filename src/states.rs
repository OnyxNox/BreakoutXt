use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Game,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MainMenuState {
    MainMenu,
    Settings,
    #[default]
    Disabled,
}
