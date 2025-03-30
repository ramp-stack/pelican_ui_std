use rust_on_rails::prelude::*;
use crate::elements::icon::{Icon, CircleIcon, CircleIconStyle};
use crate::elements::shapes::{Circle, Outline};
use crate::layout::{Stack, Offset, Size};
use crate::PelicanUI;

#[derive(Clone, Debug)]
pub enum AvatarContent {
    Icon(&'static str, CircleIconStyle),
    Image(resources::Image)
}

#[derive(Clone, Debug, Component)]
pub struct Avatar(Stack, Option<CircleIcon>, Option<Image>, Option<Shape>, Option<Flair>);
impl Events for Avatar {}

impl Avatar {
    pub fn new(ctx: &mut Context, content: AvatarContent, flair: Option<(&'static str, CircleIconStyle)>, outline: bool, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;

        let (circle_icon, image) = match content {
            AvatarContent::Image(image) => (None, Some(Image(ShapeType::Ellipse(0, (size, size)), image, None))),
            AvatarContent::Icon(name, style) => (Some(CircleIcon::new(ctx, name, style, size)), None)
        };

        Avatar(
            Stack(Offset::End, Offset::End, Size::Fit, Size::Fit),
            circle_icon,
            image,
            outline.then(|| Outline::circle(size, black)),
            flair.map(|(name, style)| Flair::new(ctx, name, style, (size as f32 / 3.0).round() as u32))
        )
    }
}

#[derive(Clone, Debug, Component)]
pub struct Flair(Stack, CircleIcon, Shape);
impl Events for Flair {}

impl Flair {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        Flair(
            Stack::center(),
            CircleIcon::new(ctx, name, style, size), 
            Outline::circle(size, black)
        )
    }
}