use rust_on_rails::prelude::*;
use crate::{ Child, Row, COLORS, ZERO, Align, Stack };
use crate::theme::fonts::{Text, TextSize};

#[derive(Clone, Copy)]
pub struct Button {
    label: &'static str,
    size: Size,
    width: Width,
    style: ButtonStyle,
    photo: Option<CircleIconData>,
    icon_l: Option<Icon>,
    icon_r: Option<Icon>
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>>  = vec![];

        let (text_size, height, size, x_padding, spacing, radius) = match self.1 {
            Size::Medium => (TextSize::md(), 32, 16, 12, 4, 12),
            Size::Large => (TextSize::lg(), 48, 24, 24, 12, 24)
        };
        
        if let Some(icon) = self.icon_l { children.push(icon.build(size, COLORS.text.heading)); }

        children.push(Text::label(ctx, self.label, text_size));

        if let Some(icon) = self.icon_r { children.push(icon.build(size, COLORS.text.heading)); }

        let content = Row(ZERO, spacing, Align::Center, children);

        let colors = COLORS.button.colors_from(self.style, ButtonState::Default);
        
        let width = match self.2 {
            Width::Hug => {
                let bound = Rect::new(0, 0, max_size.x, max_size.y);
                content.build(ctx, bound).size(ctx).x + (x_padding * 2)
            },
            Width::Expand => max_size.x,
        };

        Stack(ZERO, Align::Center, vec![
            RoundedRectangle(width, height, radius, colors.background, Some((colors.outline, 1))),
            content,
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Large,
    Medium,
}

impl Button {
    pub fn primary(label: &'static str) -> Self {
        Self {
            label,
            size: Size::Large,
            width: Width::Expand,
            style: ButtonStyle::Primary,
            photo: None,
            icon_l: None,
            icon_r: None
        }
    }

    pub fn secondary(
        label: &'static str, 
        l: Option<Icon>, 
        r: Option<Icon>
    ) -> Self {
        Self {
            label,
            size: Size::Medium,
            width: Width::Hug,
            style: ButtonStyle::Secondary,
            photo: None,
            icon_l: Some(l),
            icon_r: Some(r)
        }
    }

    pub fn ghost(label: &'static str) -> Self {
        Self {
            label,
            size: Size::Medium,
            width: Width::Hug,
            style: ButtonStyle::Ghost,
            photo: None,
            icon_l: None,
            icon_r: None
        }
    }

    pub fn keypad(label: &'static str) -> Self {
        Self {
            label,
            size: Size::Large,
            width: Width::Expand,
            style: ButtonStyle::Ghost,
            photo: None,
            icon_l: None,
            icon_r: None
        }
    }

    pub fn photo(
        label: &'static str, 
        photo: RgbaImage,
    ) -> Self {
        Self {
            label,
            size: Size::Medium,
            width: Width::Hug,
            style: ButtonStyle::Secondary,
            photo: Some(CircleIconData::Photo(photo)),
            icon_l: None,
            icon_r: None
        }
    }

    pub fn button_row(a: &'static str, b: &'static str) -> Row {
        Row(ZERO, 16, Align::Center, vec![Self::primary(a), Self::primary(b)])
    }

    pub fn quick_actions(btns: Vec<(Icon, &'static str)>) -> Wrap {
        let children = btns
            .into_iter()
            .map(|btn| {
                Self::secondary(btn.1, Some(btn.1), None)
            }).collect();

        Wrap(ZERO, 8, Align::Left, children)
    }

    pub fn quick_deselect(btns: Vec<&'static str>) -> Wrap {
        let children = btns
            .into_iter()
            .map(|label| {
                Self::secondary(label, None, Some(Icon::Close))
            }).collect();

        Wrap(ZERO, 8, Align::Left, children)
    }
}