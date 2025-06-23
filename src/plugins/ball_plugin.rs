use bevy::prelude::*;

use crate::{
    components::{Ball, Velocity},
    constants::BALL_DIAMETER,
    states::GameState,
    utility::despawn,
};

/// Color of the ball.
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

/// Initial velocity of the ball.
const BALL_INITIAL_VELOCITY: Vec2 = Vec2::new(0.5, -0.5);

/// Speed of the ball.
const BALL_SPEED: f32 = 400.0;

/// Starting position of the ball.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);

/// Collection of ball logic and configuration.
pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_ball)
            .add_systems(OnExit(GameState::Game), despawn::<Ball>);
    }
}

impl BallPlugin {
    /// Initializes the ball.
    fn setup_ball(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            Ball,
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(BALL_COLOR)),
            Transform::from_translation(BALL_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.0)),
            Velocity(BALL_INITIAL_VELOCITY.normalize() * BALL_SPEED),
        ));
    }
}
