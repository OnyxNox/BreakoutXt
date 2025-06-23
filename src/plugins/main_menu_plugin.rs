use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{components::*, constants::*, states::*, utility::despawn_screen};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(OnEnter(GameState::MainMenu), Self::enable_main_menu)
            .add_systems(OnEnter(MainMenuState::MainMenu), Self::setup_main_menu)
            .add_systems(
                Update,
                (Self::button_system, Self::menu_action).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(MainMenuState::MainMenu), despawn_screen::<MainMenu>);
    }
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

impl MainMenuPlugin {
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                    MENU_PRESSED_BUTTON.into()
                }
                (Interaction::Hovered, Some(_)) => MENU_HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => MENU_HOVERED_BUTTON.into(),
                (Interaction::None, None) => MENU_NORMAL_BUTTON.into(),
            }
        }
    }

    fn enable_main_menu(mut menu_state: ResMut<NextState<MainMenuState>>) {
        menu_state.set(MainMenuState::MainMenu);
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MainMenuAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MainMenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MainMenuAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MainMenuState::Disabled);
                    }
                    MainMenuAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
        }
    }

    fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        let button_node = Node {
            align_items: AlignItems::Center,
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(20.0)),
            width: Val::Px(300.0),
            ..default()
        };

        let button_icon_node = Node {
            left: Val::Px(10.0),
            position_type: PositionType::Absolute,
            width: Val::Px(30.0),
            ..default()
        };

        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        let exit_icon = asset_server.load("Kenney/game_icons/exitLeft.png");
        let play_icon = asset_server.load("Kenney/game_icons/forward.png");

        commands.spawn((
            Node {
                align_items: AlignItems::Center,
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            MainMenu,
            children![(
                Node {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Text::new("Breakout XT"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(MENU_NORMAL_BUTTON),
                        MainMenuAction::Play,
                        children![
                            (ImageNode::new(play_icon), button_icon_node.clone()),
                            (
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(MENU_NORMAL_BUTTON),
                        MainMenuAction::Quit,
                        children![
                            (ImageNode::new(exit_icon), button_icon_node),
                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR)),
                        ]
                    ),
                ]
            )],
        ));
    }
}
