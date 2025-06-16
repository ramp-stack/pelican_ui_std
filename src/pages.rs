use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::{Stack, Content, Header, Bumper, Page, Button, Offset, TextStyle, Text, Brand, NavigateEvent, AppPage};

use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Component)]
pub struct Error(Stack, Page, #[skip] Box<dyn AppPage>);
impl OnEvent for Error {}

impl AppPage for Error {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Ok(self.2) }
}

impl Error {
    pub fn new(ctx: &mut Context, error: &str, home: Box<dyn AppPage>) -> Self {
        let theme = &ctx.theme;
        let illustration = theme.brand.illustrations.get("error").clone();
        let font_size = theme.fonts.size;
        let illustration = Brand::new(illustration, (300.0, 300.0));
        let title = Text::new(ctx, "Something went wrong.", TextStyle::Heading, font_size.h4, Align::Left);
        let text = Text::new(ctx, error, TextStyle::Primary, font_size.lg, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(illustration), Box::new(title), Box::new(text)]);
        let button = Button::primary(ctx, "Go Home", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, None, "", None);
        Error(Stack::default(), Page::new(header, content, Some(bumper)), home)
    }
}

// #[derive(Debug, Component)]
// pub struct Splash(Stack, Page, #[skip] ApplicationPage);
// impl OnEvent for Splash {}
// impl AppPage for Splash {
//     fn navigate(self, _index: u8) -> ApplicationPage {self.2}
// }

// impl Splash {
//     pub fn new(ctx: &mut Context, home: Box<dyn AppPage>) -> ApplicationPage {
//         let wordmark = ctx.theme.brand.wordmark.clone();
//         let wordmark = Brand::new(wordmark, (300.0, 150.0));
//         let content = Content::new(Offset::Center, vec![Box::new(wordmark)]);

//         let header = Header::stack(ctx, None, "", None);
//         Error(Stack::default(), Page::new(header, content, None))
//     }
// }