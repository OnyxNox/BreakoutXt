use bevy::prelude::*;

use crate::{components::*, constants::*, states::*};

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), Self::setup_bricks);
    }
}

impl BrickPlugin {
    fn setup_bricks(mut commands: Commands) {
        let paddle_y = WALL_POSITION_BOTTOM + PADDLE_Y_PADDING;
        let total_width_of_bricks =
            (WALL_POSITION_RIGHT - WALL_POSITION_LEFT) - 2.0 * BRICK_FIELD_PADDING;
        let bottom_edge_of_bricks = paddle_y + BRICK_FIELD_PADDLE_PADDING;
        let total_height_of_bricks =
            WALL_POSITION_TOP - bottom_edge_of_bricks - BRICK_FIELD_PADDING;
        let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + BRICK_PADDING)).floor() as usize;
        let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + BRICK_PADDING)).floor() as usize;
        let n_vertical_gaps = n_columns - 1;
        let center_of_bricks = (WALL_POSITION_LEFT + WALL_POSITION_RIGHT) / 2.0;
        let left_edge_of_bricks = center_of_bricks
            - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
            - n_vertical_gaps as f32 / 2.0 * BRICK_PADDING;
        let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.0;
        let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.0;

        for row in 0..n_rows {
            for column in 0..n_columns {
                let brick_position = Vec2::new(
                    offset_x + column as f32 * (BRICK_SIZE.x + BRICK_PADDING),
                    offset_y + row as f32 * (BRICK_SIZE.y + BRICK_PADDING),
                );

                commands.spawn((
                    Brick,
                    Collider,
                    Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                ));
            }
        }
    }
}
