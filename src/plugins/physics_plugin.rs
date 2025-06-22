use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{components::*, constants::*, enums::*, events::*, resources::*};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            FixedUpdate,
            (Self::apply_velocity, Self::check_for_collisions).chain(),
        );
    }
}

impl PhysicsPlugin {
    fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_secs();
            transform.translation.y += velocity.y * time.delta_secs();
        }
    }

    fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&bounding_box) {
            return None;
        }

        let closest = bounding_box.closest_point(ball.center());
        let offset = ball.center() - closest;
        let side = if offset.x.abs() > offset.y.abs() {
            if offset.x < 0.0 {
                Collision::Left
            } else {
                Collision::Right
            }
        } else if offset.y > 0.0 {
            Collision::Top
        } else {
            Collision::Bottom
        };

        Some(side)
    }

    fn check_for_collisions(
        mut commands: Commands,
        mut score: ResMut<Score>,
        ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
        collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        let (mut ball_velocity, ball_transform) = ball_query.into_inner();

        for (collider_entity, collider_transform, maybe_brick) in &collider_query {
            let collision = Self::ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.0),
                Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_transform.scale.truncate() / 2.0,
                ),
            );

            if let Some(collision) = collision {
                collision_events.write_default();

                if maybe_brick.is_some() {
                    commands.entity(collider_entity).despawn();
                    **score += 1;
                }

                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                }

                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}
