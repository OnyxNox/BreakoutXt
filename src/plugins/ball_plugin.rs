use bevy::prelude::*;

use crate::{components::*, constants::*, states::*};

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_ball);
    }
}

impl BallPlugin {
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
