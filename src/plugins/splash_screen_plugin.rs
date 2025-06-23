use bevy::prelude::*;

use crate::{
    states::GameState,
    utility::{despawn, remove_resource},
};

/// Collection of splash screen logic and configuration.
pub struct SplashScreenPlugin;
impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), Self::setup_splash_screen)
            .add_systems(
                Update,
                Self::update_splash_screen.run_if(in_state(GameState::SplashScreen)),
            )
            .add_systems(
                OnExit(GameState::SplashScreen),
                (
                    remove_resource::<SplashScreenTimer>,
                    despawn::<SplashScreen>,
                ),
            );
    }
}

/// Splash screen marker component.
#[derive(Component)]
struct SplashScreen;

/// Splash screen display timer resource.
#[derive(Deref, DerefMut, Resource)]
struct SplashScreenTimer(Timer);

impl SplashScreenPlugin {
    /// Initializes the splash screen.
    fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Node {
                align_items: AlignItems::Center,
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            SplashScreen,
            children![(
                ImageNode::new(asset_server.load("Bevy/branding/icon.png")),
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
            )],
        ));

        commands.insert_resource(SplashScreenTimer(Timer::from_seconds(1.4, TimerMode::Once)));
    }

    /// Updates the splash screen timer and transitions to the next game state when finished.
    fn update_splash_screen(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashScreenTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::MainMenu);
        }
    }
}
