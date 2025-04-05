use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{OutlinedRectangle, Rectangle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::components::button::ButtonState;
use crate::layout::{Column, Stack, Bin, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct Card(Stack, OutlinedRectangle, CardContent, #[skip] ButtonState, #[skip] fn(&mut Context) -> ());
impl Card {
    pub fn new(
        ctx: &mut Context,
        avatar: AvatarContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        let (bg, oc) = (colors.background.primary, colors.outline.secondary);
        let background = OutlinedRectangle::new(bg, oc, 16, 1);
        let content = CardContent::new(ctx, avatar, title, subtitle, description);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(|widths: Vec<(u32, u32)>| (widths[1].0, u32::MAX)), 
            Size::custom(|heights: Vec<(u32, u32)>| heights[1]), 
            Padding::default()
        );

        Card(layout, background, content, ButtonState::Default, on_click)
    }
}

impl Events for Card {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        if let Some(state) = self.3.handle(ctx, event) {
            *self.1.background() = match state {
                ButtonState::Default => colors.background.primary,
                ButtonState::Disabled => colors.background.primary,
                ButtonState::Selected => colors.background.primary,
                ButtonState::Hover => colors.background.secondary,
            }
        }
        if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
            match self.3 {
                ButtonState::Default | ButtonState::Hover => (self.4)(ctx),
                _ => {}
            }
        }
        false
    }
}

#[derive(Debug, Component)]
pub struct CardContent(Column, Avatar, BasicText, BasicText, Bin<Stack, Rectangle>, BasicText);
impl Events for CardContent {}

impl CardContent {
    fn new(
        ctx: &mut Context, 
        avatar: AvatarContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str
    ) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (font_size, color) = (theme.fonts.size, theme.colors.outline.secondary);
        CardContent(
            Column(8, Offset::Center, Size::Fit, Padding(16, 16, 16, 16)),
            Avatar::new(ctx, avatar, None, false, 64),
            Text::new(ctx, title, TextStyle::Heading, font_size.h3),
            Text::new(ctx, subtitle, TextStyle::Primary, font_size.xs),
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Fit, Size::Static(1), Padding(0, 6, 0, 6)), 
                Rectangle::new(color)
            ),
            Text::new(ctx, description, TextStyle::Primary, font_size.sm),
        )
    }
}

// let card = Card {
//     circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
//     title: "Donkey Lovers",
//     subtitle: "101 members",
//     description: "A place for donkey lovers to converse.",
// }