use bevy::prelude::*;

use crate::traits::MenuAction;

/// Ball marker component.
#[derive(Component)]
pub struct Ball;

/// Brick marker component.
#[derive(Component)]
pub struct Brick;

/// Collider marker component.
#[derive(Component, Default)]
pub struct Collider;

/// Main menu marker component.
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuAction {
    Play,
    Options,
    Exit,
    Back,
}

impl MenuAction for MainMenuAction {}

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct ButtonPressed;
