use rust_on_rails::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BrandResources {
    pub wordmark: resources::Image,
    pub logomark: resources::Image,
    pub app_icon: resources::Image,
    pub illustrations: Illustrations
}

impl BrandResources {
    pub fn new(
        logomark: resources::Image, 
        wordmark: resources::Image,
        app_icon: resources::Image,
        illustrations: Illustrations
    ) -> Self {
        BrandResources { logomark, wordmark, app_icon, illustrations }
    }

    pub fn default(ctx: &mut Context) -> Self {
        BrandResources{
            logomark: ctx.add_svg(&ctx.load_file("brand/logomark.svg").unwrap(), 8.0),
            wordmark: ctx.add_svg(&ctx.load_file("brand/wordmark.svg").unwrap(), 8.0),
            app_icon: ctx.add_svg(&ctx.load_file("brand/app_icon.svg").unwrap(), 8.0),
            illustrations: Illustrations::default(ctx),
        }
    }
} 

#[derive(Clone, Debug)]
pub struct Illustrations(HashMap<&'static str, resources::Image>);

impl Illustrations {
    pub fn default(ctx: &mut Context) -> Self {
        let mut illustrations = HashMap::new();

        illustrations.insert("dodo", ctx.add_svg(&ctx.load_file("brand/illustrations/dodo.svg").unwrap(), 8.0));
        illustrations.insert("hummingbird", ctx.add_svg(&ctx.load_file("brand/illustrations/hummingbird.svg").unwrap(), 8.0));
        illustrations.insert("toucan", ctx.add_svg(&ctx.load_file("brand/illustrations/toucan.svg").unwrap(), 8.0));
        illustrations.insert("emu", ctx.add_svg(&ctx.load_file("brand/illustrations/emu.svg").unwrap(), 8.0));

        Self(illustrations)
    }

    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| panic!("Could not find illustration {:?}", name)).clone()
    }

    pub fn add_icon(&mut self, name: &'static str, illustration: resources::Image) {
        if let Some(existing) = self.0.get_mut(&name) {
            *existing = illustration; 
        } else {
            self.0.insert(name, illustration);
        }
    }
}
