use bevy::prelude::*;

#[derive(Resource)]
pub struct Borders {
    pub red_border: Handle<Image>,
    pub red_flat: Handle<Image>,
    pub red_gloss: Handle<Image>,
}

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct Fonts {
    pub narrow: Handle<Font>,
    pub normal: Handle<Font>,
}

#[derive(Deref, DerefMut, Resource)]
pub struct Score(pub usize);

#[derive(Resource)]
pub struct UserInterface {
    pub borders: Borders,
    pub fonts: Fonts,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
