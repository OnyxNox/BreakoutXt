use bevy::prelude::*;

pub const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub const BALL_DIAMETER: f32 = 30.0;

pub const BALL_INITIAL_VELOCITY: Vec2 = Vec2::new(0.5, -0.5);

pub const BALL_SPEED: f32 = 400.0;

pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);

pub const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

pub const BRICK_FIELD_PADDING: f32 = 20.0;

pub const BRICK_FIELD_PADDLE_PADDING: f32 = 270.0;

pub const BRICK_PADDING: f32 = 5.0;

pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);

pub const PADDLE_Y_PADDING: f32 = 60.0;

pub const WALL_POSITION_BOTTOM: f32 = -300.0;

pub const WALL_POSITION_LEFT: f32 = -450.0;

pub const WALL_POSITION_RIGHT: f32 = 450.0;

pub const WALL_POSITION_TOP: f32 = 300.0;

pub const WALL_THICKNESS: f32 = 10.0;
