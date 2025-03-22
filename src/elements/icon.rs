use rust_on_rails::prelude::*;
use crate::PelicanUI;

#[derive(Clone)]
pub struct Icon(pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        Icon(Image(ShapeType::Rectangle(0, (size, size)), theme.icons.get(name), Some(color)))
    }
}

impl Component for Icon {
    fn build(&mut self, _ctx: &mut Context, _max_size: (u32, u32)) -> Container { 
        container![&mut self.0] 
    }
}
