use bevy::prelude::*;

/// Ball marker component.
#[derive(Component)]
pub struct Ball;

/// Brick marker component.
#[derive(Component)]
pub struct Brick;

/// Collider marker component.
#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuAction {
    Play,
    Settings,
    Quit,
    Back,
}

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
