use rust_on_rails::prelude::*;
use crate::PelicanUI;

#[derive(Clone)]
pub struct Icon(pub &'static str, pub Color, pub u32);

impl Component for Icon {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let theme = &ctx.get::<PelicanUI>().theme;
        Container(Offset::default(), Size::Static(self.2, self.2), vec![
            Box::new(Image(ShapeType::Rectangle(0), theme.icons.get(&self.0), Some(self.1)))
        ])
    }
}
