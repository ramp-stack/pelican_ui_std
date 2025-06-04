use pelican_ui::events::{OnEvent, MouseState, Event, MouseEvent};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{OutlinedRectangle, Rectangle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::components::button::ButtonState;
use crate::layout::{Column, Stack, Bin, Padding, Offset, Size};

#[derive(Debug, Component)]
pub struct Card(Stack, OutlinedRectangle, CardContent, #[skip] ButtonState, #[skip] fn(&mut Context) -> ());
impl Card {
    pub fn new(
        ctx: &mut Context,
        avatar: AvatarContent, 
        title: &str, 
        subtitle: &str, 
        description: &str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let colors = ctx.theme.colors;
        let (bg, oc) = (colors.background.primary, colors.outline.secondary);
        let background = OutlinedRectangle::new(bg, oc, 16.0, 1.0);
        let content = CardContent::new(ctx, avatar, title, subtitle, description);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(|widths: Vec<(f32, f32)>| (widths[1].0, f32::MAX)), 
            Size::custom(|heights: Vec<(f32, f32)>| heights[1]), 
            Padding::default()
        );

        Card(layout, background, content, ButtonState::Default, on_click)
    }
}

impl OnEvent for Card {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.3 {
                    ButtonState::Default | ButtonState::Hover => (self.4)(ctx),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

#[derive(Debug, Component)]
struct CardContent(Column, Avatar, Text, Text, Bin<Stack, Rectangle>, Text);
impl OnEvent for CardContent {}

impl CardContent {
    fn new(
        ctx: &mut Context, 
        avatar: AvatarContent, 
        title: &str, 
        subtitle: &str, 
        description: &str
    ) -> Self {
        let theme = &ctx.theme;
        let (font_size, color) = (theme.fonts.size, theme.colors.outline.secondary);
        CardContent(
            Column::new(8.0, Offset::Center, Size::Fit, Padding(16.0, 16.0, 16.0, 16.0)),
            Avatar::new(ctx, avatar, None, false, 64.0, None),
            Text::new(ctx, title, TextStyle::Heading, font_size.h3, Align::Left),
            Text::new(ctx, subtitle, TextStyle::Primary, font_size.xs, Align::Left),
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Fit, Size::Static(1.0), Padding(0.0, 6.0, 0.0, 6.0)), 
                Rectangle::new(color)
            ),
            Text::new(ctx, description, TextStyle::Primary, font_size.sm, Align::Left),
        )
    }
}

// let card = Card {
//     circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
//     title: "Donkey Lovers",
//     subtitle: "101 members",
//     description: "A place for donkey lovers to converse.",
// }
