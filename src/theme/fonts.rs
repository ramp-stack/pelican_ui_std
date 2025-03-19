use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;

// pub struct FontResources {
//     pub style: TextStyle,
//     pub size: TextSize,
// }

// impl FontResources {
//     pub fn new(ctx: &mut Context) -> Self {
//         FontResources{
//             style: TextStyle::new(ctx),
//             size: TextSize::default()
//         }
//     }
// } 

pub struct Text(pub BasicText);

impl Text {
    pub fn new(text: &'static str, color: &'static str, size: u32, font: Handle) -> Self {
        Self(BasicText(text, color, size, (size as f32*1.25) as u32, font))
    }

    pub fn heading(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.text.heading, size.0, TextStyle::new(ctx).heading)
    }

    pub fn primary(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.text.primary, size.0, TextStyle::new(ctx).text)
    }

    pub fn primary_white(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.text.heading, size.0, TextStyle::new(ctx).text)
    }

    pub fn secondary(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.text.secondary, size.0, TextStyle::new(ctx).text)
    }

    pub fn error(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.status.danger, size.0, TextStyle::new(ctx).text)
    }

    pub fn label(ctx: &mut Context, text: &'static str, size: TextSize) -> Self {
        Self::new(text, crate::COLORS.text.heading, size.0, TextStyle::new(ctx).label)
    }
}

impl ComponentBuilder for Text {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Copy, Clone)]
pub struct TextSize(u32);

impl TextSize {
    pub fn title() -> Self {Self(72)}
    pub fn h1() -> Self {Self(48)}
    pub fn h2() -> Self {Self(32)}
    pub fn h3() -> Self {Self(24)}
    pub fn h4() -> Self {Self(20)}
    pub fn h5() -> Self {Self(16)}
    pub fn h6() -> Self {Self(14)}
    pub fn xl() -> Self {Self(24)}
    pub fn lg() -> Self {Self(20)}
    pub fn md() -> Self {Self(16)}
    pub fn sm() -> Self {Self(14)}
    pub fn xs() -> Self {Self(12)}
}

pub struct TextStyle {
    pub heading: Handle,
    pub text: Handle,
    pub label: Handle,
}

impl TextStyle {
    pub fn new(ctx: &mut Context) -> Self {
        let bold = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let regular = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        Self {
            heading: bold.clone(),
            text: regular.clone(),
            label: bold.clone(),
        }
    }
}
