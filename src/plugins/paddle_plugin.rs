use bevy::prelude::*;

use crate::{components::*, constants::*, states::*, utility::despawn};

/// Color of the paddle.
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

/// Size of the paddle.
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);

/// Speed of the paddle.
const PADDLE_SPEED: f32 = 500.0;

/// Horizontal padding between the paddle and walls.
const PADDLE_X_PADDING: f32 = 10.0;

/// Collection of paddle logic and configuration.
pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_paddle)
            .add_systems(
                FixedUpdate,
                Self::update_paddle.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn::<Paddle>);
    }
}

/// Paddle marker component.
#[derive(Component)]
struct Paddle;

impl PaddlePlugin {
    /// Initializes the paddle.
    fn setup_paddle(mut commands: Commands) {
        commands.spawn((
            Collider,
            Paddle,
            Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(0.0, WALL_POSITION_BOTTOM + PADDLE_Y_PADDING, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
        ));
    }

    /// Updates the paddle position in response to user inputs.
    fn update_paddle(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    ) {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        let new_paddle_position =
            paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

        let left_bound =
            WALL_POSITION_LEFT + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_X_PADDING;
        let right_bound =
            WALL_POSITION_RIGHT - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_X_PADDING;

        paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
    }
}
