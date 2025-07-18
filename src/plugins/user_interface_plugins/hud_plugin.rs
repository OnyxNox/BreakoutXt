use bevy::prelude::*;

use crate::{resources::*, states::*, utility::despawn};

/// Font size of the Heads-Up Display (HUD).
const HUD_FONT_SIZE: f32 = 33.0;

/// Color of the Heads-Up Display (HUD) score text.
const HUD_SCORE_TEXT_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

/// Color of the Heads-Up Display (HUD) text.
const HUD_TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

/// Padding of the Heads-Up Display (HUD).
const HUD_TEXT_PADDING: Val = Val::Px(9.0);

/// Collection of Heads-Up Display (HUD) logic and configuration.
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_hud)
            .add_systems(Update, Self::update_hud.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn::<HeadsUpDisplay>);
    }
}

/// Heads-Up Display (HUD) marker component.
#[derive(Component)]
pub struct HeadsUpDisplay;

impl HudPlugin {
    /// Initializes the Heads-Up Display (HUD).
    fn setup_hud(mut commands: Commands) {
        commands.insert_resource(Score(0));

        commands.spawn((
            HeadsUpDisplay,
            Node {
                position_type: PositionType::Absolute,
                top: HUD_TEXT_PADDING,
                left: HUD_TEXT_PADDING,
                ..Default::default()
            },
            Text::new("Score: "),
            TextColor(HUD_TEXT_COLOR),
            TextFont {
                font_size: HUD_FONT_SIZE,
                ..Default::default()
            },
            children![(
                TextColor(HUD_SCORE_TEXT_COLOR),
                TextFont {
                    font_size: HUD_FONT_SIZE,
                    ..Default::default()
                },
                TextSpan::default(),
            )],
        ));
    }

    /// Updates the Heads-Up Display (HUD) to display the current score.
    fn update_hud(
        score: Res<Score>,
        score_entity: Single<Entity, (With<HeadsUpDisplay>, With<Text>)>,
        mut text_writer: TextUiWriter,
    ) {
        *text_writer.text(*score_entity, 1) = score.to_string();
    }
}
