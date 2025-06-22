use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Deref, DerefMut, Resource)]
pub struct Score(pub usize);
