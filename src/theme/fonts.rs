use rust_on_rails::prelude::*;

#[derive(Clone)]
pub struct FontResources {
    pub fonts: Fonts,
    pub size: FontSize,
}

impl FontResources {
    pub fn new(fonts: Fonts, size: FontSize) -> Self {
        Self { fonts, size }
    }

    pub fn default(ctx: &mut Context) -> Self {
        FontResources{
            fonts: Fonts::default(ctx),
            size: FontSize::default()
        }
    }
} 

#[derive(Clone)]
pub struct Fonts {
    pub heading: resources::Font,
    pub text: resources::Font,
    pub label: resources::Font,
    pub keyboard: resources::Font,
    pub emoji: resources::Font,
}

impl Fonts {
    pub fn new(
        heading: resources::Font, 
        text: resources::Font, 
        label: resources::Font, 
        keyboard: resources::Font, 
        emoji: resources::Font
    ) -> Self {
        Self { heading, text, label, keyboard, emoji }
    }

    pub fn default(ctx: &mut Context) -> Self {
        println!("loading font resources");
        let bold = resources::Font::new(ctx, ctx.load_file("fonts/outfit_bold.ttf").unwrap());
        let medium = resources::Font::new(ctx, ctx.load_file("fonts/outfit_medium.ttf").unwrap());
        let regular = resources::Font::new(ctx, ctx.load_file("fonts/outfit_regular.ttf").unwrap());
        let emoji = resources::Font::new(ctx, ctx.load_file("fonts/noto_color_emoji.ttf").unwrap());
        Self {
            heading: bold.clone(),
            text: regular.clone(),
            label: bold.clone(),
            keyboard: medium.clone(),
            emoji: emoji.clone()
        }
    }
}


#[derive(Copy, Clone)]
pub struct FontSize {
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
    pub xs: f32
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize {
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
            xs: 12.0
        }
    }
}