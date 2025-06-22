use bevy::prelude::*;

use crate::{events::*, resources::*, states::*};

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            Self::play_collision_sound.run_if(in_state(GameState::Game)),
        )
        .add_systems(OnEnter(GameState::Game), Self::setup_audio);
    }
}

impl AudioPlugin {
    fn play_collision_sound(
        mut commands: Commands,
        mut collision_events: EventReader<CollisionEvent>,
        sound: Res<CollisionSound>,
    ) {
        if !collision_events.is_empty() {
            collision_events.clear();

            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }

    fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
        let ball_collision_sound =
            asset_server.load("Kenney/impact_sounds/footstep_concrete_002.ogg");

        commands.insert_resource(CollisionSound(ball_collision_sound));
    }
}
