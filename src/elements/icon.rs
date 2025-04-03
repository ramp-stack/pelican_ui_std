use rust_on_rails::prelude::*;
use crate::layout::Stack;
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct Icon(Stack, pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let icon = ctx.get::<PelicanUI>().theme.icons.get(name);
        Icon(Stack::default(), Image(ShapeType::Rectangle(0, (size, size)), icon, Some(color)))
    }

    pub fn color(&mut self) -> &mut Option<Color> { self.1.color() }
}
impl Events for Icon {}
