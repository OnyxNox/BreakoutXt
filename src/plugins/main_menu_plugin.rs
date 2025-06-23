use bevy::{color::palettes::css::CRIMSON, ecs::spawn::SpawnWith, prelude::*};

use crate::{components::*, constants::*, resources::Volume, states::*, utility::despawn_screen};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .insert_resource(Volume(7))
            .add_systems(OnEnter(GameState::MainMenu), Self::enable_main_menu)
            .add_systems(OnEnter(MainMenuState::MainMenu), Self::setup_main_menu)
            .add_systems(
                OnEnter(MainMenuState::Settings),
                Self::sound_settings_menu_setup,
            )
            .add_systems(
                Update,
                Self::setting_button::<Volume>.run_if(in_state(MainMenuState::Settings)),
            )
            .add_systems(
                Update,
                (Self::button_system, Self::menu_action).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(MainMenuState::Settings),
                despawn_screen::<SettingsMenu>,
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
                    MainMenuAction::Settings => {
                        menu_state.set(MainMenuState::Settings);
                    }
                    MainMenuAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MainMenuAction::Back => {
                        menu_state.set(MainMenuState::MainMenu);
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
        let settings_icon = asset_server.load("Kenney/game_icons/gear.png");

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
                        button_node.clone(),
                        BackgroundColor(MENU_NORMAL_BUTTON),
                        MainMenuAction::Settings,
                        children![
                            (ImageNode::new(settings_icon), button_icon_node.clone()),
                            (
                                Text::new("Settings"),
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

    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
        let button_node = Node {
            align_items: AlignItems::Center,
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(20.0)),
            width: Val::Px(200.0),
            ..default()
        };
        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        let volume = *volume;
        let button_node_clone = button_node.clone();
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SettingsMenu,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                        Children::spawn((
                            Spawn((Text::new("Volume"), button_text_style.clone())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_node_clone.clone()
                                        },
                                        BackgroundColor(MENU_NORMAL_BUTTON),
                                        Volume(volume_setting),
                                    ));

                                    if volume == Volume(volume_setting) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(MENU_NORMAL_BUTTON),
                        MainMenuAction::Back,
                        children![(Text::new("Back"), button_text_style)]
                    )
                ]
            )],
        ));
    }

    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        let (previous_button, mut previous_button_color) = selected_query.into_inner();

        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                *previous_button_color = MENU_NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }
}
