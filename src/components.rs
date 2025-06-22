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

/// Paddle marker component.
#[derive(Component)]
pub struct Paddle;

/// Physics velocity vector.
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
