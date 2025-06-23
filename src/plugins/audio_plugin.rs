use bevy::prelude::*;

use crate::{
    events::CollisionEvent, resources::CollisionSound, states::GameState, utility::remove_resource,
};

/// Collection of audio logic and configuration.
pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_audio)
            .add_systems(
                FixedUpdate,
                Self::handle_collision_events.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), remove_resource::<CollisionSound>);
    }
}

impl AudioPlugin {
    /// Plays collision sound for all collision events.
    fn handle_collision_events(
        mut commands: Commands,
        mut collision_events: EventReader<CollisionEvent>,
        collision_sound: Res<CollisionSound>,
    ) {
        if !collision_events.is_empty() {
            collision_events.clear();

            commands.spawn((
                AudioPlayer(collision_sound.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }

    /// Initializes audio resources.
    fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
        let collision_sound = asset_server.load("Kenney/impact_sounds/footstep_concrete_002.ogg");

        commands.insert_resource(CollisionSound(collision_sound));
    }
}
