use rust_on_rails::prelude::*;
use crate::PelicanUI;
use crate::layout::{Stack, Offset, Size};

#[derive(Clone, Debug, Component)]
pub struct Icon(Stack, pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        Icon(
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit),
            Image(ShapeType::Rectangle(0, (size, size)), theme.icons.get(name), Some(color))
        )
    }

    pub fn set_color(&mut self, color: Color) {
        let Image(_, _, c) = &mut self.1;
        *c = Some(color);
    }
}
impl Events for Icon {}
