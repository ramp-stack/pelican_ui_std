use rust_on_rails::prelude::*;
use crate::PelicanUI;
use crate::layout::{Stack, Offset, Size};

#[derive(Clone, Debug)]
pub struct Icon(Stack, pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        Icon(
            Stack((Offset::Start, Offset::Start), (Size::Fit, Size::Fit)),
            Image(ShapeType::Rectangle(0, (size, size)), theme.icons.get(name), Some(color))
        )
    }
}

impl Component for Icon {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1]}
    fn children(&self) -> Vec<&ComponentRef> {vec![&self.1]}
    fn layout(&self) -> &dyn Layout {&self.0}
}