use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::{Outline, Circle};
use crate::layout::{Stack, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug)]
pub enum AvatarContent {
    Icon(&'static str, AvatarIconStyle),
    Image(resources::Image)
}

#[derive(Debug)]
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
impl Events for Avatar {}

impl Avatar {
    pub fn new(ctx: &mut Context, content: AvatarContent, flair: Option<(&'static str, AvatarIconStyle)>, outline: bool, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;

        let (circle_icon, image) = match content {
            AvatarContent::Image(image) => (None, Some(Image{shape: ShapeType::Ellipse(0, (size, size)), image, color: None})),
            AvatarContent::Icon(name, style) => (Some(AvatarIcon::new(ctx, name, style, size)), None)
        };

        Avatar(
            Stack(Offset::End, Offset::End, Size::Fit, Size::Fit, Padding::default()),
            circle_icon,
            image,
            outline.then(|| Outline::circle(size, black)),
            flair.map(|(name, style)| Flair::new(ctx, name, style, (size as f32 / 3.0).round() as u32))
        )
    }
}

#[derive(Debug, Component)]
struct AvatarIcon(Stack, Shape, Image);
impl Events for AvatarIcon {}

impl AvatarIcon {
    fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: u32) -> Self {
        let icon_size = (size as f32 * 0.75).round() as u32;
        let (background, icon_color) = style.get(ctx);
        AvatarIcon(
            Stack::center(),
            Circle::new(size - 2, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}

#[derive(Debug, Component)]
struct Flair(Stack, AvatarIcon, Shape);
impl Events for Flair {}

impl Flair {
    fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        Flair(
            Stack::center(),
            AvatarIcon::new(ctx, name, style, size),
            Outline::circle(size, black)
        )
    }
}
