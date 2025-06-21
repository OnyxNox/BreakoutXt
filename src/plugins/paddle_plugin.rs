use bevy::prelude::*;

use crate::{
    components::{Collider, Paddle},
    plugins::wall_plugin::WALL_POSITION_BOTTOM,
};

/// Paddle color.
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

/// Paddle padding from the bottom wall.
const PADDLE_PADDING: f32 = 60.0;

/// Paddle size.
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);

/// Collection of resources and systems around the paddle.
pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_paddle);
    }
}

impl PaddlePlugin {
    /// Spawns paddle.
    fn setup_paddle(mut commands: Commands) {
        commands.spawn((
            Collider,
            Paddle,
            Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(0.0, WALL_POSITION_BOTTOM + PADDLE_PADDING, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
        ));
    }
}
