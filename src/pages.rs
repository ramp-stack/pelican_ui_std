use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::{Stack, Content, Header, Bumper, Page, Button, Offset, TextStyle, Text, Brand, NavigateEvent, AppPage};

#[derive(Debug, Component)]
pub struct Error(Stack, Page, #[skip] Box<dyn AppPage>);
impl OnEvent for Error {}

impl AppPage for Error {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Ok(self.2) }
}

impl Error {
    pub fn new(ctx: &mut Context, error: &str, home: Box<dyn AppPage>) -> Self {
        let theme = &ctx.theme;
        let illustration = theme.brand.illustrations.get("error").unwrap();
        let font_size = theme.fonts.size;
        let illustration = Brand::new(illustration, (300.0, 300.0));
        let title = Text::new(ctx, "Something went wrong.", TextStyle::Heading, font_size.h4, Align::Left);
        let text = Text::new(ctx, error, TextStyle::Primary, font_size.md, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(illustration), Box::new(title), Box::new(text)]);
        let button = Button::primary(ctx, "Go Back", move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let bumper = Bumper::single_button(ctx, button);
        let header = Header::stack(ctx, None, "", None);
        Error(Stack::default(), Page::new(Some(header), content, Some(bumper)), home)
    }
}

#[derive(Debug, Component)]
pub struct Splash(Stack, Page);
impl OnEvent for Splash {}
impl AppPage for Splash {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Ok(self) }
}

impl Splash {
    pub fn new(ctx: &mut Context) -> Self {
        let wordmark = ctx.theme.brand.wordmark.clone();
        let content = Content::new(Offset::Center, vec![Box::new(Brand::new(wordmark, (162.0, 34.5)))]);

        Splash(Stack::default(), Page::new(None, content, None))
    }
}

#[derive(Debug, Component)]
pub struct PelicanHome(Stack, Page);
impl OnEvent for PelicanHome {}

impl AppPage for PelicanHome {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl PelicanHome {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.theme;
        let logo = theme.brand.logomark.clone();
        let font_size = theme.fonts.size;
        let illustration = Brand::new(logo, (150.0, 150.0));
        let title = Text::new(ctx, "Welcome to Pelican UI", TextStyle::Heading, font_size.h4, Align::Center);
        let text = Text::new(ctx, "featherlight ui for heavy ideas", TextStyle::Primary, font_size.md, Align::Center);
        let content = Content::new(Offset::Center, vec![Box::new(illustration), Box::new(title), Box::new(text)]);
        PelicanHome(Stack::default(), Page::new(None, content, None))
    }
}