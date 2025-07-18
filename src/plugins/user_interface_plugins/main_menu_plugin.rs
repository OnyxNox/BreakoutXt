use bevy::{color::palettes::css::CRIMSON, ecs::spawn::SpawnWith, prelude::*};

use crate::{
    bundles::UiButton,
    components::*,
    constants::*,
    resources::{UserInterface, Volume},
    states::*,
    utility::despawn,
};

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
                Self::menu_action.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(MainMenuState::Settings), despawn::<SettingsMenu>)
            .add_systems(OnExit(MainMenuState::MainMenu), despawn::<MainMenu>);
    }
}

impl MainMenuPlugin {
    fn enable_main_menu(mut menu_state: ResMut<NextState<MainMenuState>>) {
        menu_state.set(MainMenuState::MainMenu);
    }

    fn menu_action(
        mut game_state: ResMut<NextState<GameState>>,
        mut menu_state: ResMut<NextState<MainMenuState>>,
        interaction_query: Query<
            (&Interaction, &MainMenuAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MainMenuAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MainMenuState::Disabled);
                    }
                    MainMenuAction::Options => {
                        menu_state.set(MainMenuState::Settings);
                    }
                    MainMenuAction::Exit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MainMenuAction::Back => {
                        menu_state.set(MainMenuState::MainMenu);
                    }
                }
            }
        }
    }

    fn setup_main_menu(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        ui: Res<UserInterface>,
    ) {
        let button_text_font = TextFont {
            font: ui.fonts.narrow.clone(),
            font_size: 24.0,
            ..default()
        };

        let exit_icon = asset_server.load("Kenney/game_icons/exit_left.png");
        let play_icon = asset_server.load("Kenney/game_icons/forward.png");
        let options_icon = asset_server.load("Kenney/game_icons/gear.png");

        let border_slicer = TextureSlicer {
            border: BorderRect::all(16.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 0.5,
        };

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
                children![
                    (
                        ImageNode {
                            image: ui.borders.red_flat.clone(),
                            image_mode: NodeImageMode::Sliced(border_slicer.clone()),
                            ..Default::default()
                        },
                        children![(
                            Node {
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::px(0.0, 0.0, 6.0, 10.0),
                                width: Val::Px(486.0),
                                ..default()
                            },
                            children![(
                                Text::new("Breakout XT"),
                                TextColor(TEXT_COLOR),
                                TextFont {
                                    font: ui.fonts.normal.clone(),
                                    font_size: 28.0,
                                    ..default()
                                },
                            )],
                        )],
                    ),
                    (
                        ImageNode {
                            image: ui.borders.red_border.clone(),
                            image_mode: NodeImageMode::Sliced(border_slicer.clone()),
                            ..Default::default()
                        },
                        Node {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::top(Val::Px(-9.0)),
                            width: Val::Px(486.0),
                            ..default()
                        },
                        children![
                            UiButton::new(
                                Some("Play"),
                                Some(play_icon),
                                button_text_font.clone(),
                                ui.borders.red_gloss.clone(),
                                MainMenuAction::Play
                            ),
                            UiButton::new(
                                Some("Options"),
                                Some(options_icon),
                                button_text_font.clone(),
                                ui.borders.red_gloss.clone(),
                                MainMenuAction::Options
                            ),
                            UiButton::new(
                                Some("Exit"),
                                Some(exit_icon),
                                button_text_font.clone(),
                                ui.borders.red_gloss.clone(),
                                MainMenuAction::Exit
                            ),
                        ]
                    )
                ],
            )],
        ));
    }

    fn sound_settings_menu_setup(
        mut commands: Commands,
        volume: Res<Volume>,
        ui: Res<UserInterface>,
    ) {
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
        let button_image = ui.borders.red_gloss.clone();
        let back_button_image = ui.borders.red_flat.clone();

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
                                        ImageNode {
                                            image: button_image.clone(),
                                            image_mode: NodeImageMode::Sliced(TextureSlicer {
                                                border: BorderRect::all(16.0),
                                                center_scale_mode: SliceScaleMode::Stretch,
                                                sides_scale_mode: SliceScaleMode::Stretch,
                                                max_corner_scale: 0.5,
                                            }),
                                            color: MENU_NORMAL_BUTTON,
                                            ..Default::default()
                                        },
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
                        ImageNode {
                            image: back_button_image,
                            image_mode: NodeImageMode::Sliced(TextureSlicer {
                                border: BorderRect::all(16.0),
                                center_scale_mode: SliceScaleMode::Stretch,
                                sides_scale_mode: SliceScaleMode::Stretch,
                                max_corner_scale: 0.5,
                            }),
                            color: MENU_NORMAL_BUTTON,
                            ..Default::default()
                        },
                        MainMenuAction::Back,
                        children![(Text::new("Back"), button_text_style)]
                    )
                ]
            )],
        ));
    }

    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        selected_query: Single<(Entity, &mut ImageNode), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        let (previous_button, mut previous_button_image) = selected_query.into_inner();

        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                previous_button_image.color = MENU_NORMAL_BUTTON;
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }
}
