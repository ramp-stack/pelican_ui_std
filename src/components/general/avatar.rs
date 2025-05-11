use rust_on_rails::prelude::*;
use crate::elements::images::Icon;
use crate::elements::shapes::{Outline, Circle};
use crate::layout::{Stack, Offset, Size, Row, Padding};
use crate::PelicanUI;

#[derive(Debug, Clone)]
pub enum AvatarContent {
    Icon(&'static str, AvatarIconStyle),
    Image(resources::Image)
}

#[derive(Debug, Copy, Clone)]
pub enum AvatarIconStyle {
    Primary,
    Secondary,
    Brand,
    Success,
    Warning,
    Danger
}

impl AvatarIconStyle {
    fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            AvatarIconStyle::Primary => (colors.text.heading, colors.background.primary),
            AvatarIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            AvatarIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            AvatarIconStyle::Success => (colors.status.success, colors.text.heading),
            AvatarIconStyle::Warning => (colors.status.warning, colors.text.heading),
            AvatarIconStyle::Danger => (colors.status.danger, colors.text.heading),
        }
    }
}

#[derive(Debug, Component)]
pub struct Avatar(Stack, Option<AvatarIcon>, Option<Image>, Option<Shape>, Option<Flair>);
impl OnEvent for Avatar {}

impl Avatar {
    pub fn new(ctx: &mut Context, content: AvatarContent, flair: Option<(&'static str, AvatarIconStyle)>, outline: bool, size: f32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;

        let (circle_icon, image) = match content {
            AvatarContent::Image(image) => (None, Some(Image{shape: ShapeType::Ellipse(0.0, (size, size)), image, color: None})),
            AvatarContent::Icon(name, style) => (Some(AvatarIcon::new(ctx, name, style, size)), None)
        };

        Avatar(
            Stack(Offset::End, Offset::End, Size::Fit, Size::Fit, Padding::default()),
            circle_icon,
            image,
            outline.then(|| Outline::circle(size, black)),
            flair.map(|(name, style)| Flair::new(ctx, name, style, size / 3.0))
        )
    }
}

#[derive(Debug, Component)]
struct AvatarIcon(Stack, Shape, Image);
impl OnEvent for AvatarIcon {}

impl AvatarIcon {
    fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32) -> Self {
        let icon_size = size * 0.75;
        let (background, icon_color) = style.get(ctx);
        AvatarIcon(
            Stack::center(),
            Circle::new(size - 2.0, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}

#[derive(Debug, Component)]
struct Flair(Stack, AvatarIcon, Shape);
impl OnEvent for Flair {}

impl Flair {
    fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        Flair(
            Stack::center(),
            AvatarIcon::new(ctx, name, style, size),
            Outline::circle(size, black)
        )
    }
}

#[derive(Debug, Component)]
pub struct AvatarRow(Row, Vec<Avatar>);
impl OnEvent for AvatarRow {}

impl AvatarRow {
    pub fn new(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        AvatarRow(
            Row::center(0.0), 
            avatars.into_iter().map(|avatar| Avatar::new(ctx, avatar, None, true, 32.0)).collect()
        )
    }
}