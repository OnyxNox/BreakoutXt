use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Brick;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
