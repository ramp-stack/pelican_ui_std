#[derive(Debug, Component)]
pub struct InfoCard(Column, Text, Text);
impl OnEvent for InfoCard {}

impl InfoCard {
    pub fn new(ctx: &mut Context, title: &str, description: &str) -> Self {
        let font_size = ctx.theme.fonts.size;

        InfoCard(
            Column::new(8.0, Offset::Center, Size::Fit, Padding::new(16.0)),
            Text::new(ctx, title, TextStyle::Header, font_size.h3, Align::Left),
            Text::new(ctx, description, TextStyle::Primary, font_size.md, Align::Left)
        )
    }
}