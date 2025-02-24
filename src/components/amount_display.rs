use rust_on_rails::prelude::*;
use crate::{ Child, Row, Column, COLORS, ZERO, Align, ConstrainedBox };
use crate::theme::fonts::{Text, TextSize};

pub struct AmountDisplay(pub &'static str, pub Option<&'static str>);

impl ComponentBuilder for AmountDisplay {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let mut column: Vec<(Box<dyn ComponentBuilder>, bool)> 
            = vec![(Child!(Text::heading(ctx, self.0, TextSize::h1())), false)];

        column.push((
            match self.1.is_some() {
                true => Child!(Row { spacing: 8, align: Align::Left, padding: ZERO, children: vec![
                    (Child!(Shape(ShapeType::Rectangle(32, 32), COLORS.status.danger, None)), false),
                    (Child!(Text::error(ctx, self.1.unwrap(), TextSize::lg())), false)
                ]}),
                false => Child!(Text::secondary(ctx, "0.00001234 BTC", TextSize::lg()))
            }, 
            false
        ));

        ConstrainedBox!(300, 
            Column { spacing: 32, align: Align::Center, padding: Vec2::new(16, 32), children: column }
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}