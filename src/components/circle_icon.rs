use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::{Circle, Outline};
use crate::layout::{Stack, Offset, Size};
use crate::PelicanUI;

pub enum ProfileImage {
    Icon(&'static str, CircleIconStyle),
    Image(resources::Image)
}

pub enum _ProfileImage {
    Icon(CircleIconData),
    Image(Image)
}

impl _ProfileImage {
    pub fn new(ctx: &mut Context, input: ProfileImage, s: u32) -> Self {
        match input {
            ProfileImage::Image(image) => _ProfileImage::Image(Image(ShapeType::Ellipse(0, (s, s)), image, None)),
            ProfileImage::Icon(name, style) => _ProfileImage::Icon(CircleIconData::new(ctx, name, style, s))
        }
    }
}

pub struct CircleIcon(pub _ProfileImage, pub Option<Shape>, pub Option<Container<'static>>);

impl CircleIcon {
    pub fn new(ctx: &mut Context, content: ProfileImage, flair: Option<(&'static str, CircleIconStyle)>, outline: bool, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        let flair_s = (size as f32 / 3.0).round() as u32;

        CircleIcon(
            _ProfileImage::new(ctx, content, size), 
            outline.then(|| Outline::new(size, black)),
            flair.map(|(name, style)| Container::new(Stack(Offset::Center, Size::default()), vec![
                &mut CircleIconData::new(ctx, name, style, flair_s), &mut Outline::new(flair_s, black)
            ]))
        )
    }
}

impl Component for CircleIcon {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![];

        match &mut self.0 {
            _ProfileImage::Icon(icon) => children.push(icon),
            _ProfileImage::Image(image) => children.push(image)
        }

        if let Some(outline) = &mut self.1 { children.push(outline); }
        if let Some(flair) = &mut self.2 {
            children.push(flair);
        }

        Container::new(Stack(Offset::BottomRight, Size::default()), children)
    }
}

pub struct CircleIconData(pub Shape, pub Icon);

impl CircleIconData {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        println!("circle icon data size: {:?}", size);
        let icon_size = (size as f32 * 0.75).round() as u32;
        println!("icon_size: {:?}", icon_size);
        let (background, icon_color) = style.get(ctx);
        CircleIconData(
            Circle::new(size - 2, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}

impl Component for CircleIconData {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container::new(Stack(Offset::Center, Size::default()), vec![&mut self.0, &mut self.1])
    }
}

pub enum CircleIconStyle {
    Primary,
    Secondary,
    Brand,
    Success,
    Warning,
    Danger
}

impl CircleIconStyle {
    pub fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            CircleIconStyle::Primary => (colors.text.heading, colors.background.primary),
            CircleIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            CircleIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            CircleIconStyle::Success => (colors.status.success, colors.text.heading),
            CircleIconStyle::Warning => (colors.status.warning, colors.text.heading),
            CircleIconStyle::Danger => (colors.status.danger, colors.text.heading),
        }
    }
}