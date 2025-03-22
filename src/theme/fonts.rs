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
    pub emoji: resources::Font,
}

impl Fonts {
    pub fn new(heading: resources::Font, text: resources::Font, label: resources::Font, emoji: resources::Font) -> Self {
        Self { heading, text, label, emoji }
    }

    pub fn default(ctx: &mut Context) -> Self {
        println!("loading font resources");
        let bold = resources::Font::new(ctx, ctx.load_file("fonts/outfit_bold.ttf").unwrap());
        let regular = resources::Font::new(ctx, ctx.load_file("fonts/outfit_regular.ttf").unwrap());
        let emoji = resources::Font::new(ctx, ctx.load_file("fonts/noto_color_emoji.ttf").unwrap());
        Self {
            heading: bold.clone(),
            text: regular.clone(),
            label: bold.clone(),
            emoji: emoji.clone()
        }
    }
}


#[derive(Copy, Clone)]
pub struct FontSize {
    pub title: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub h5: u32,
    pub h6: u32,
    pub xl: u32,
    pub lg: u32,
    pub md: u32,
    pub sm: u32,
    pub xs: u32
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize {
            title: 72,
            h1: 48,
            h2: 32,
            h3: 24,
            h4: 20,
            h5: 16,
            h6: 14,
            xl: 24,
            lg: 20,
            md: 16,
            sm: 14,
            xs: 12
        }
    }
}