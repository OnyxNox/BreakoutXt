use bevy::prelude::*;

/// Heads-Up Display text font size.
const FONT_SIZE: f32 = 33.0;

/// Heads-Up Display score text color.
const SCORE_TEXT_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

/// Heads-Up Display text color.
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

/// Heads-Up Display text padding.
const TEXT_PADDING: Val = Val::Px(9.0);

/// Collection of resources and systems around the Heads-Up Display.
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, Self::setup_hud)
            .add_systems(Update, Self::update_hud);
    }
}

/// Game score resource.
#[derive(Deref, DerefMut, Resource)]
pub struct Score(usize);

/// Heads-Up Display marker component.
#[derive(Component)]
pub struct HeadsUpDisplay;

impl HudPlugin {
    /// Spawns the Heads-Up Display.
    fn setup_hud(mut commands: Commands) {
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

    /// Updates Heads-Up Display's score.
    fn update_hud(
        score: Res<Score>,
        score_root: Single<Entity, (With<HeadsUpDisplay>, With<Text>)>,
        mut text_writer: TextUiWriter,
    ) {
        *text_writer.text(*score_root, 1) = score.to_string();
    }
}
