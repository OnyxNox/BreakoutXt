use bevy::prelude::*;

/// Ball color.
pub const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

/// Ball size.
pub const BALL_DIAMETER: f32 = 30.0;

/// Ball initial velocity.
pub const BALL_INITIAL_VELOCITY: Vec2 = Vec2::new(0.5, -0.5);

/// Ball rate of speed.
pub const BALL_SPEED: f32 = 400.0;

/// Ball starting position.
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);

/// Brick color.
pub const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

/// Brick field padding from the walls.
pub const BRICK_FIELD_PADDING: f32 = 20.0;

/// Padding between the brick field and the paddle.
pub const BRICK_FIELD_PADDLE_PADDING: f32 = 270.0;

/// Padding between bricks.
pub const BRICK_PADDING: f32 = 5.0;

/// Brick size.
pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);

/// Paddle padding from the bottom wall.
pub const PADDLE_Y_PADDING: f32 = 60.0;

/// Bottom wall y-axis position.
pub const WALL_POSITION_BOTTOM: f32 = -300.0;

/// Left wall x-axis position.
pub const WALL_POSITION_LEFT: f32 = -450.0;

/// Right wall x-axis position.
pub const WALL_POSITION_RIGHT: f32 = 450.0;

/// Top wall y-axis position.
pub const WALL_POSITION_TOP: f32 = 300.0;

/// Wall thickness.
pub const WALL_THICKNESS: f32 = 10.0;
