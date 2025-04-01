use rust_on_rails::prelude::Context;

use colors::ColorResources;
use fonts::FontResources;
use icons::IconResources;
use brand::BrandResources;

pub mod colors;
pub mod fonts;
pub mod icons;
pub mod brand;

pub struct Theme {
    pub colors: ColorResources,
    pub fonts: FontResources,
    pub icons: IconResources,
    pub brand: BrandResources,
}

impl Theme {
    pub fn default(ctx: &mut Context) -> Self {
        Theme {
            colors: ColorResources::default(),
            fonts: FontResources::default(ctx),
            icons: IconResources::default(ctx),
            brand: BrandResources::default(ctx)
        }
    }

    pub fn new(
        colors: ColorResources, 
        fonts: FontResources, 
        icons: IconResources,
        brand: BrandResources,
    ) -> Self { Theme { colors, fonts, icons, brand } }
}
