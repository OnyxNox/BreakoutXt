use bevy::prelude::*;

use crate::{components::Brick, states::GameState};

/// Collection of overarching game logic and configuration.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            Self::handle_game_end_condition.run_if(in_state(GameState::Game)),
        );
    }
}

impl GamePlugin {
    /// Transitions to the next game state when all bricks have been destroyed.
    fn handle_game_end_condition(
        mut game_state: ResMut<NextState<GameState>>,
        bricks_query: Query<Entity, With<Brick>>,
    ) {
        if bricks_query.is_empty() {
            game_state.set(GameState::MainMenu);
        }
    }
}
