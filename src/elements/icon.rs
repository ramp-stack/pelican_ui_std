use rust_on_rails::prelude::*;
use crate::PelicanUI;

pub struct Icon(pub &'static str, pub Color, pub u32);

impl ComponentBuilder for Icon {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let theme = &ctx.get::<PelicanUI>().theme;
        Image(ShapeType::Rectangle(0, (self.2, self.2)), theme.icons.get(&self.0), Some(self.1)).build_children(ctx, max_size)
    }
}
