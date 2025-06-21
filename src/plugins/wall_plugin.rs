use bevy::prelude::*;

use crate::components::Collider;

/// Bottom wall y-axis position.
pub const WALL_POSITION_BOTTOM: f32 = -300.0;

/// Wall color.
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

/// Left wall x-axis position.
const WALL_POSITION_LEFT: f32 = -450.0;

/// Right wall x-axis position.
const WALL_POSITION_RIGHT: f32 = 450.0;

/// Top wall y-axis position.
const WALL_POSITION_TOP: f32 = 300.0;

/// Wall thickness.
const WALL_THICKNESS: f32 = 10.0;

/// Collection of resources and systems around walls.
pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_walls);
    }
}

/// Wall marker component.
#[derive(Component)]
#[require(Collider, Sprite, Transform)]
pub struct Wall;
impl Wall {
    /// Initialize a new wall.
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..Default::default()
            },
        )
    }
}

/// Represents a wall location.
enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    /// Returns the wall location's world position.
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(WALL_POSITION_LEFT, 0.0),
            WallLocation::Right => Vec2::new(WALL_POSITION_RIGHT, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, WALL_POSITION_BOTTOM),
            WallLocation::Top => Vec2::new(0.0, WALL_POSITION_TOP),
        }
    }

    /// Returns the wall location's world size.
    fn size(&self) -> Vec2 {
        let arena_width = WALL_POSITION_RIGHT - WALL_POSITION_LEFT;
        let arena_height = WALL_POSITION_TOP - WALL_POSITION_BOTTOM;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallPlugin {
    /// Spawns walls.
    fn setup_walls(mut commands: Commands) {
        commands.spawn(Wall::new(WallLocation::Left));
        commands.spawn(Wall::new(WallLocation::Right));
        commands.spawn(Wall::new(WallLocation::Top));
        commands.spawn(Wall::new(WallLocation::Bottom));
    }
}
