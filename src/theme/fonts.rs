use rust_on_rails::prelude::*;

pub struct FontResources {
    pub style: TextStyle,
    pub size: TextSize,
}

impl FontResources {
    pub fn new(ctx: &mut ComponentContext) -> Self {
        FontResources{
            style: TextStyle::new(ctx),
            size: TextSize::default()
        }
    }
} 

pub struct TextStyle {
    pub heading: Handle,
    pub text: Handle,
    pub label: Handle,
}

impl TextStyle {
    pub fn new(ctx: &mut ComponentContext) -> Self {
        let bold = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let regular = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        Self {
            heading: bold.clone(),
            text: regular.clone(),
            label: bold.clone(),
        }
    }
}

pub struct TextSize {
    pub title: f32,
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
    pub h4: f32,
    pub h5: f32,
    pub h6: f32,
    pub xl: f32,
    pub lg: f32,
    pub md: f32,
    pub sm: f32,
    pub xs: f32,
}

impl Default for TextSize {
    fn default() -> TextSize {
        TextSize {
            title: 72.0,
            h1: 48.0,
            h2: 32.0,
            h3: 24.0,
            h4: 20.0,
            h5: 16.0,
            h6: 14.0,
            xl: 24.0,
            lg: 20.0,
            md: 16.0,
            sm: 14.0,
            xs: 12.0,
        }
    }
}