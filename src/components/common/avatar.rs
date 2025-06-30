use pelican_ui::events::{OnEvent, MouseState, Event, MouseEvent};
use pelican_ui::drawable::{Drawable, Component, Image, Color, Shape, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component, resources};

use crate::elements::images::Icon;
use crate::elements::shapes::{Outline, Circle};
use crate::layout::{Stack, Offset, Size, Row, Padding};
use crate::utils::Callback;

#[derive(Component)]
pub struct Avatar(Stack, PrimaryAvatar, Option<Flair>, #[skip] pub Option<Callback>);

impl Avatar {
    pub fn new(
        ctx: &mut Context, 
        content: AvatarContent, 
        flair: Option<(&'static str, AvatarIconStyle)>, 
        outline: bool, 
        size: f32,
        on_click: Option<Callback>
    ) -> Self {
        let black = ctx.theme.colors.shades.black;

        Avatar(
            Stack(Offset::End, Offset::End, Size::Fit, Size::Fit, Padding::default()),
            PrimaryAvatar::new(ctx, content, outline, size),
            flair.map(|(name, style)| Flair::new(ctx, name, style, size / 3.0, black)),
            on_click
        )
    }

    pub fn set_content(&mut self, content: AvatarContent)  {self.1.set_content(content)}
    pub fn flair(&mut self) -> &mut Option<Flair> {&mut self.2}
    pub fn outline(&mut self) -> &mut Option<Shape> {&mut self.1.3}
    pub fn avatar(&mut self) -> &mut PrimaryAvatar {&mut self.1}
}

impl OnEvent for Avatar {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent{state: MouseState::Pressed, position: Some(_)}) = event.as_any_mut().downcast_mut::<MouseEvent>() {
            if let Some(on_click) = &mut self.3 {
                ctx.hardware.haptic();
                (on_click)(ctx)
            }
        }
        false
    }
}

impl std::fmt::Debug for Avatar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Avatar...")
    }
}

#[derive(Component, Debug)]
pub struct PrimaryAvatar(Stack, Option<AvatarIcon>, Option<Image>, Option<Shape>);
impl OnEvent for PrimaryAvatar {}

impl PrimaryAvatar {
    fn new(ctx: &mut Context, content: AvatarContent, outline: bool, size: f32) -> Self {
        let black = ctx.theme.colors.shades.black;

        let (circle_icon, image) = match content {
            AvatarContent::Image(image) => (None, Some(Image{shape: ShapeType::Ellipse(0.0, (size, size)), image, color: None})),
            AvatarContent::Icon(name, style) => (Some(AvatarIcon::new(ctx, name, style, size)), None)
        };

        PrimaryAvatar(
            Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default()),
            circle_icon, image, outline.then(|| Outline::circle(size, black)),
        )
    }

    pub fn set_content(&mut self, content: AvatarContent) {
        match content {
            AvatarContent::Image(image) => {
                if let Some(avatar_image) = &mut self.2 {
                    avatar_image.image = image;
                } else {
                    let size = self.1.as_mut().unwrap().1.shape.size().0 + 2.0;
                    self.2 = Some(Image{shape: ShapeType::Ellipse(0.0, (size, size)), image, color: None});
                }

                self.1 = None;
            }
            AvatarContent::Icon(_name, _style) => {/* to do */}
        };
    }

    pub fn image(&mut self) -> &mut Option<Image> { &mut self.2 }
    pub fn icon(&mut self) -> &mut Option<AvatarIcon> { &mut self.1 }
}


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
    Danger,
    Custom(Color, Color)
}

impl AvatarIconStyle {
    fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.theme.colors;
        match self {
            AvatarIconStyle::Primary => (colors.text.heading, colors.background.primary),
            AvatarIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            AvatarIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            AvatarIconStyle::Success => (colors.status.success, colors.shades.white),
            AvatarIconStyle::Warning => (colors.status.warning, colors.shades.white),
            AvatarIconStyle::Danger => (colors.status.danger, colors.shades.white),
            AvatarIconStyle::Custom(background, icon) => (*background, *icon)
        }
    }
}
#[derive(Debug, Component)]
pub struct AvatarIcon(Stack, Shape, Image);

impl OnEvent for AvatarIcon {}

impl AvatarIcon {
    pub fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32) -> Self {
        let icon_size = size * 0.75;
        let (background, icon_color) = style.get(ctx);
        AvatarIcon(
            Stack::center(),
            Circle::new(size - 2.0, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
    pub fn icon(&mut self) -> &mut Image { &mut self.2 }
}

#[derive(Debug, Component)]
pub struct Flair(Stack, AvatarIcon, Shape);

impl OnEvent for Flair {}

impl Flair {
    pub fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32, color: Color) -> Self {
        Flair(
            Stack::center(),
            AvatarIcon::new(ctx, name, style, size),
            Outline::circle(size, color)
        )
    }

    pub fn icon(&mut self) -> &mut Image {self.1.icon()}
}

#[derive(Debug, Component)]
pub struct AvatarRow(Row, Vec<Avatar>);

impl OnEvent for AvatarRow {}

impl AvatarRow {
    pub fn new(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        AvatarRow(
            Row::center(-16.0),
            avatars.into_iter().take(5).map(|avatar| Avatar::new(ctx, avatar, None, true, 32.0, None)).collect()
        )
    }

    pub fn update(&mut self, ctx: &mut Context, avatars: Vec<AvatarContent>) {
        self.1 = avatars.into_iter().take(5).map(|avatar| Avatar::new(ctx, avatar, None, true, 32.0, None)).collect()
    }

    pub fn count(&mut self) -> usize {self.1.len()}
}
