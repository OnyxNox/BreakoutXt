use bevy::prelude::*;

use crate::{states::*, utility::despawn_screen};

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), Self::splash_setup)
            .add_systems(Update, Self::countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashScreen>);
    }
}

#[derive(Component)]
struct SplashScreen;

#[derive(Deref, DerefMut, Resource)]
struct SplashTimer(Timer);

impl SplashPlugin {
    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("Bevy/branding/icon.png");

        commands.spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            SplashScreen,
            children![(
                ImageNode::new(icon),
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
            )],
        ));

        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::MainMenu);
        }
    }
}
