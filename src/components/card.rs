use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{RoundedRectangle, Outline, ExpandingRoundedRectangle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::components::button::ButtonState;
use crate::layout::{Row, Column, Stack, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct Card(Stack, CardBackground, CardContent, #[skip] ButtonState, #[skip] fn(&mut Context, (u32, u32)) -> ());
impl Card {
    pub fn new(
        ctx: &mut Context,
        avatar: AvatarContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str,
        on_click: fn(&mut Context, (u32, u32)) -> (),
    ) -> Self {
        let padding = 16;
        let content = CardContent::new(ctx, avatar, title, subtitle, description, padding);
        let height = content.size(ctx).min_height().0+(padding*2);
        let width = Size::Fill(content.size(ctx).min_width(), MaxSize::MAX);
        let background = CardBackground::new(ctx, width, height);

        Card(Stack::center(), background, content, ButtonState::Default, on_click)
    }

    pub fn set_state(&mut self, ctx: &mut Context, state: ButtonState) {
        if self.3 != state {
            self.3 = state;
            let theme = &ctx.get::<PelicanUI>().theme;
            let color = match state {
                ButtonState::Hover => theme.colors.background.secondary,
                _ => theme.colors.background.primary,
            };
            self.1.set_color(color);
        }
    }
}

impl Events for Card {
    fn on_click(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        if let Some(position) = position {(self.4)(ctx, position);}
        false
    }
    fn on_move(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        // println!("move: {:?}", position);
        match (position.is_some(), self.3) {
            (true, ButtonState::Default) => self.set_state(ctx, ButtonState::Hover),
            (false, ButtonState::Hover) => self.set_state(ctx, ButtonState::Default),
            _ => {}
        };
        false
    }
}


#[derive(Clone, Debug, Component)]
pub struct CardContent(Column, Avatar, BasicText, BasicText, SeparationLine, BasicText);
impl Events for CardContent {}

impl CardContent {
    fn new(
        ctx: &mut Context, 
        avatar: AvatarContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str,
        padding: u32,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        CardContent(
            // Column(8, Offset::Center, Size::Fit, Padding(padding, 0, padding, 0)),
            Column::center(8),
            Avatar::new(ctx, avatar, None, false, 64),
            Text::new(ctx, title, TextStyle::Heading, font_size.h3),
            Text::new(ctx, subtitle, TextStyle::Primary, font_size.xs),
            SeparationLine::new(ctx, padding),
            Text::new(ctx, description, TextStyle::Primary, font_size.sm),
        )
    }
}

#[derive(Clone, Debug, Component)]
pub struct CardBackground(Stack, Shape, Shape);

impl CardBackground {
    pub fn new(ctx: &mut Context, width: Size, height: u32) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        CardBackground(
            Stack(Offset::Center, Offset::Center, width, Size::Fit, Padding::default()),
            RoundedRectangle::new(100, height, 16, colors.background.primary),
            Outline::rounded_rectangle(100, height, 16, 1, colors.outline.secondary)
        )
    }

    fn set_color(&mut self, bg: Color) {
        let Shape(_, c) = &mut self.1;
        *c = bg;
    }
}

impl Events for CardBackground {
    fn on_resize(&mut self, _ctx: &mut Context, size: (u32, u32)) {
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.1 {
            *w = size.0;
        }
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.2 {
            *w = size.0;
        }
    }
}

#[derive(Clone, Debug, Component)]
struct SeparationLine(Stack, ExpandingRoundedRectangle);
impl Events for SeparationLine {}

impl SeparationLine {
    pub fn new(ctx: &mut Context, p: u32) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        SeparationLine(
            Stack(Offset::Start, Offset::Start, Size::Fill(MinSize(10), MaxSize::MAX), Size::Fit, Padding(p, 6, p, 6)),
            ExpandingRoundedRectangle::new(ctx, 1, 0, color)
        )
    }
}


// let card = Card {
//     circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
//     title: "Donkey Lovers",
//     subtitle: "101 members",
//     description: "A place for donkey lovers to converse.",
// }