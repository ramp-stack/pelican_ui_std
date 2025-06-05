use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::{AppPage, Stack, Content, Header, Bumper, Page, Button, Offset, TextStyle, Text, Brand};

#[derive(Debug, Component, AppPage)]
pub struct Error(Stack, Page, #[skip] bool);
impl OnEvent for Error {}

impl Error {
    pub fn new(ctx: &mut Context) -> Self {
        let error = "Error";
        let theme = &ctx.theme;
        let illustration = theme.brand.illustrations.get("error").clone();
        let font_size = theme.fonts.size;
        let illustration = Brand::new(illustration, (300.0, 300.0));
        let title = Text::new(ctx, "Something went wrong.", TextStyle::Heading, font_size.h4, Align::Left);
        let text = Text::new(ctx, error, TextStyle::Primary, font_size.lg, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(illustration), Box::new(title), Box::new(text)]);

        let button = Button::primary(ctx, "Try Again", move |_ctx: &mut Context| {
            // ctx.state().get::<HomePage>().go()
        });

        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, None, "", None);
        Error(Stack::default(), Page::new(header, content, Some(bumper)), false)
    }
}