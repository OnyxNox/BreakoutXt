use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    SplashScreen,
    MainMenu,
    Game,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MainMenuState {
    #[default]
    Disabled,
    MainMenu,
    Settings,
}
