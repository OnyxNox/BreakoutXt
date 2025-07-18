use bevy::prelude::*;

use crate::{components::*, constants::*};

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::button_system);
    }
}

impl ButtonPlugin {
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut ImageNode, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut image_node, selected) in &mut interaction_query {
            let color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => MENU_PRESSED_BUTTON,
                (Interaction::Hovered, Some(_)) => MENU_HOVERED_PRESSED_BUTTON,
                (Interaction::Hovered, None) => MENU_HOVERED_BUTTON,
                (Interaction::None, None) => MENU_NORMAL_BUTTON,
            };
            image_node.color = color;
        }
    }
}
