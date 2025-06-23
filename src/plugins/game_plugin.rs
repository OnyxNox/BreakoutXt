use bevy::prelude::*;

use crate::{components::*, states::*};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Self::check_game_end);
    }
}

impl GamePlugin {
    fn check_game_end(
        mut game_state: ResMut<NextState<GameState>>,
        bricks_query: Query<Entity, With<Brick>>,
    ) {
        if bricks_query.is_empty() {
            game_state.set(GameState::MainMenu);
        }
    }
}
