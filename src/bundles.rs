use bevy::prelude::*;

use crate::{constants::TEXT_COLOR, traits::MenuAction};

#[derive(Bundle)]
pub struct UiButton<M>
where
    M: Component + MenuAction,
{
    button: Button,
    button_node: Node,
    image_node: ImageNode,
    menu_action: M,
}

impl<M> UiButton<M>
where
    M: Component + MenuAction,
{
    pub fn new(
        text: Option<impl Into<String>>,
        icon: Option<Handle<Image>>,
        text_font: TextFont,
        button_image: Handle<Image>,
        menu_action: M,
    ) -> impl Bundle {
        let border_slicer = TextureSlicer {
            border: BorderRect::all(18.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };

        let ui_button = Self {
            button: Button,
            button_node: Node {
                align_items: AlignItems::Center,
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(20.0)),
                width: Val::Px(300.0),
                ..default()
            },
            image_node: ImageNode {
                image: button_image,
                image_mode: NodeImageMode::Sliced(border_slicer),
                ..Default::default()
            },
            menu_action,
        };

        (
            ui_button,
            children![
                UiButton::<M>::icon_node(icon),
                UiButton::<M>::text_node(text, text_font),
            ],
        )
    }

    fn icon_node(icon: Option<Handle<Image>>) -> impl Bundle {
        let button_icon_node = Node {
            left: Val::Px(10.0),
            position_type: PositionType::Absolute,
            width: Val::Px(30.0),
            ..default()
        };

        match icon {
            Some(icon) => (ImageNode::new(icon), button_icon_node),
            None => (ImageNode::default(), button_icon_node),
        }
    }

    fn text_node(text: Option<impl Into<String>>, text_font: TextFont) -> impl Bundle {
        match text {
            Some(text) => (Text::new(text), text_font, TextColor(TEXT_COLOR)),
            None => (Text::default(), TextFont::default(), TextColor::default()),
        }
    }
}
