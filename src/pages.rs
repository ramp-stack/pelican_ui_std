use rust_on_rails::prelude::*;
use crate::prelude::{AppPage, Stack, Content, Header, Bumper, Page, Button, Offset, TextStyle, PelicanUI, Text, Brand, NavigateEvent};



#[derive(Debug, Component, AppPage)]
pub struct Error(Stack, Page, #[skip] bool);
impl OnEvent for Error {}

impl Error {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let text_size = theme.fonts.size.h4;
        let illustration = theme.brand.illustrations.get("error").clone();
        let illustration = Brand::new(illustration, (300.0, 300.0));
        let text = Text::new(ctx, "Something went wrong.", TextStyle::Heading, text_size, Align::Left);
        let content = Content::new(Offset::Center, vec![Box::new(illustration), Box::new(text)]);

        let button = Button::primary(ctx, "Try Again", move |_ctx: &mut Context| {
            // ctx.state().get::<HomePage>().go()
        });

        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, None, "", None);
        Error(Stack::default(), Page::new(header, content, Some(bumper)), false)
    }
}