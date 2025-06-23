use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Deref, DerefMut, Resource)]
pub struct Score(pub usize);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
