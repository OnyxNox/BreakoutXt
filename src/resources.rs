use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub struct Score(pub usize);
