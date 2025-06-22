use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Brick;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
