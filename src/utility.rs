use bevy::prelude::*;

/// Recursively despawns all entities that contain the specified component.
pub fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Removes resource from memory.
pub fn remove_resource<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}
