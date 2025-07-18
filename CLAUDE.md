# CLAUDE.md
This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Breakout XT is a Rust-based Breakout game built with the Bevy game engine. It extends the [Bevy Breakout example](https://github.com/bevyengine/bevy/blob/release-0.16.1/examples/games/breakout.rs) with additional features like a splash screen, main menu, settings, and other UI components.

## Development Commands
- **Run the game**: `cargo run`
- **Build**: `cargo build`
- **Build release**: `cargo build --release`
- **Check code**: `cargo check`
- **Run tests**: `cargo test`

## Architecture
The project uses Bevy's plugin-based architecture with a state machine for game flow:

### Core States
- `GameState`: Primary game flow (`SplashScreen` → `MainMenu` → `Game`)
- `MainMenuState`: Sub-states for menu navigation (`MainMenu`, `Settings`)

### Plugin Structure
All game functionality is organized into plugins in `src/plugins/`:
- `GamePlugins`: Main plugin group that orchestrates all other plugins
- Core gameplay: `ball_plugin.rs`, `brick_plugin.rs`, `paddle_plugin.rs`, `physics_plugin.rs`
- UI systems: `user_interface_plugins/` (button, HUD, main menu)
- Supporting: `audio_plugin.rs`, `camera_plugin.rs`, `wall_plugin.rs`

### Key Components
- Entity-Component-System (ECS) architecture with components in `src/components.rs`
- Game entities: `Ball`, `Brick`, `Paddle`
- UI entities: `MainMenu`, `SettingsMenu`, `MainMenuAction`

### Resources
- `UserInterface`: Contains UI assets (borders, fonts)
- `Borders`: UI button textures
- `Fonts`: Typography assets

### Code Organization
Source code is organized under the `src/` directory:
- `main.rs`: Entry point and app initialization
- `states.rs`: State definitions for game flow
- `components.rs`: ECS components
- `plugins.rs`: Plugin group registration
- `resources.rs`: Global resources
- `events.rs`: Custom events
- `bundles.rs`: Entity bundles
- `enums.rs`: Shared enumerations
- `traits.rs`: Shared traits
- `constants.rs`: Game constants
- `utility.rs`: Utility functions

### Asset Structure
Assets are organized in `assets/` with credits to Kenney and Bevy:
- UI assets: `Kenney/ui_pack/`
- Audio: `Kenney/impact_sounds/`
- Icons: `Kenney/game_icons/`
- Bevy branding: `Bevy/branding/`

## Development Notes
- Uses Bevy 0.16.1 with Rust 2024 edition
- Game uses state transitions for flow control
- UI is built with Bevy's built-in UI system
- Physics implemented with custom collision detection
- Audio system integrated for sound effects