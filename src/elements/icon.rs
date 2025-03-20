use rust_on_rails::prelude::*;
use crate::PelicanUI;

#[derive(Clone)]
pub struct Icon(pub &'static str, pub Color, pub u32);

impl Component for Icon {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<((u32, u32), Box<dyn Component>)> {
        let theme = &ctx.get::<PelicanUI>().theme;
        vec![Image(ShapeType::Rectangle(0, (self.2, self.2)), theme.icons.get(&self.0), Some(self.1)).stack()]
    }
}
