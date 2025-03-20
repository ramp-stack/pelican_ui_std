use rust_on_rails::prelude::Context;

pub mod colors;
pub mod fonts;
pub mod icons;

pub struct Theme {
    pub fonts: fonts::FontResources,
    pub colors: colors::ColorResources,
    pub icons: icons::IconResources,
}

impl Theme {
    pub fn new(ctx: &mut Context) -> Self {
        Theme {
            colors: colors::ColorResources::default(),
            fonts: fonts::FontResources::new(ctx),
            icons: icons::IconResources::new(ctx)
        }
    }
}
