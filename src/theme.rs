use rust_on_rails::prelude::Context;

use colors::ColorResources;
use fonts::FontResources;
use icons::IconResources;

pub mod colors;
pub mod fonts;
pub mod icons;

pub struct Theme {
    pub colors: ColorResources,
    pub fonts: FontResources,
    pub icons: IconResources,
}

impl Theme {
    pub fn default(ctx: &mut Context) -> Self {
        Theme {
            colors: ColorResources::default(),
            fonts: FontResources::default(ctx),
            icons: IconResources::default(ctx)
        }
    }

    pub fn new(
        colors: ColorResources, 
        fonts: FontResources, 
        icons: IconResources
    ) -> Self {
        Theme { colors, fonts, icons }
    }

    pub fn set_colors(&mut self, colors: ColorResources) {
        self.colors = colors;
    }

    pub fn set_fonts(&mut self, fonts: FontResources) {
        self.fonts = fonts;
    }

    pub fn set_icons(&mut self, icons: IconResources) {
        self.icons = icons;
    }
}
