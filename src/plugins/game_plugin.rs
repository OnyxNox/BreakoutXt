use bevy::prelude::*;

use crate::{
    components::Brick,
    resources::{Borders, Fonts, UserInterface},
    states::GameState,
};

/// Collection of overarching game logic and configuration.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_game).add_systems(
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

    fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(UserInterface {
            borders: Borders {
                red_border: asset_server.load("Kenney/ui_pack/button_square_border.png"),
                red_flat: asset_server.load("Kenney/ui_pack/button_square_flat.png"),
                red_gloss: asset_server.load("Kenney/ui_pack/button_square_gloss.png"),
            },
            fonts: Fonts {
                narrow: asset_server.load("Kenney/ui_pack/future_narrow.ttf"),
                normal: asset_server.load("Kenney/ui_pack/future.ttf"),
            },
        });
    }
}
