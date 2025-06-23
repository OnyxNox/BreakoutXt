use bevy::prelude::*;

use crate::{resources::*, states::*, utility::despawn_screen};

const FONT_SIZE: f32 = 33.0;

const SCORE_TEXT_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

const TEXT_PADDING: Val = Val::Px(9.0);

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_hud)
            .add_systems(Update, Self::update_hud.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<HeadsUpDisplay>);
    }
}

#[derive(Component)]
pub struct HeadsUpDisplay;

impl HudPlugin {
    fn setup_hud(mut commands: Commands) {
        commands.insert_resource(Score(0));

        commands.spawn((
            HeadsUpDisplay,
            Node {
                position_type: PositionType::Absolute,
                top: TEXT_PADDING,
                left: TEXT_PADDING,
                ..Default::default()
            },
            Text::new("Score: "),
            TextColor(TEXT_COLOR),
            TextFont {
                font_size: FONT_SIZE,
                ..Default::default()
            },
            children![(
                TextColor(SCORE_TEXT_COLOR),
                TextFont {
                    font_size: FONT_SIZE,
                    ..Default::default()
                },
                TextSpan::default(),
            )],
        ));
    }

    fn update_hud(
        score: Res<Score>,
        score_root: Single<Entity, (With<HeadsUpDisplay>, With<Text>)>,
        mut text_writer: TextUiWriter,
    ) {
        *text_writer.text(*score_root, 1) = score.to_string();
    }
}
