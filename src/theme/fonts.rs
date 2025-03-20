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
}

impl Fonts {
    pub fn new(heading: resources::Font, text: resources::Font, label: resources::Font) -> Self {
        Self { heading, text, label }
    }
    
    pub fn default(ctx: &mut Context) -> Self {
        let bold = resources::Font::new(ctx, ctx.load_file("fonts/outfit_bold.ttf").unwrap());
        let regular = resources::Font::new(ctx, ctx.load_file("fonts/outfit_regular.ttf").unwrap());
        Self {
            heading: bold.clone(),
            text: regular.clone(),
            label: bold.clone(),
        }
    }
}


#[derive(Copy, Clone)]
pub struct FontSize {
    title: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
    h5: u32,
    h6: u32,
    xl: u32,
    lg: u32,
    md: u32,
    sm: u32,
    xs: u32
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