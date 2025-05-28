use rust_on_rails::prelude::*;
use crate::prelude::*;

pub trait AppPage: Drawable + std::fmt::Debug + 'static {
    fn into_boxed(self) -> Box<dyn AppPage> where Self: Sized {
        Box::new(self) as Box<dyn AppPage>
    }
}

// dyn_clone::clone_trait_object!(AppPage);

pub use pelican_macro::AppPage as derive_AppPage;

pub mod macros {
    pub use pelican_macro::AppPage;
}

// #[derive(Debug, Component, AppPage)]
// pub struct Error(Stack, Page, #[skip] bool);
// impl OnEvent for Error {}

// impl Error {
//     fn new(ctx: &mut Context) -> Self {
//         let text_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
//         let adrs = ctx.get::<BDKPlugin>().get_new_address().to_string();
//         let qr_code = QRCode::new(ctx, &adrs);
//         let text = Text::new(ctx, "Scan to Error bitcoin.", TextStyle::Secondary, text_size, Align::Left);
//         let content = Content::new(Offset::Center, vec![Box::new(qr_code), Box::new(text)]);

//         Button::primary(ctx, "Try Again", move |ctx: &mut Context| {
//             ctx.state().get::<HomePage>().go()
//         });

//         let bumper = Bumper::single_button(ctx, button);
//         let close = IconButton::navigation(ctx, "left", |ctx: &mut Context| BitcoinHome::navigate(ctx));
//         let header = Header::stack(ctx, Some(close), "Error bitcoin", None);
//         Error(Stack::default(), Page::new(header, content, Some(bumper)), false)
//     }
// }