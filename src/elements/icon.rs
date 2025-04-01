use rust_on_rails::prelude::*;
use crate::PelicanUI;
use crate::layout::{Stack, Offset, Size};

#[derive(Clone, Debug, Component)]
pub struct Icon(Stack, pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let icon = ctx.get::<PelicanUI>().theme.icons.get(name);
        Icon(Stack::default(), Image(ShapeType::Rectangle(0, (size, size)), icon, Some(color)))
    }

    pub fn set_color(&mut self, color: Color) {
        let Image(_, _, c) = &mut self.1;
        *c = Some(color);
    }
}
impl Events for Icon {}
