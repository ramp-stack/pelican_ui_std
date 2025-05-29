use rust_on_rails::prelude::*;
use crate::Theme;

pub struct PelicanUI {
    pub theme: Theme,
}

impl PelicanUI {
    pub fn update_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

impl Plugin for PelicanUI {
    async fn background_tasks(_ctx: &mut HeadlessContext) -> Tasks {
        vec![]
    }

    async fn new(ctx: &mut Context, _h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        ctx.include_assets(include_assets!("./resources"));
        (PelicanUI { theme: Theme::default(ctx) }, vec![])
    }
}

// state that takes in a return page, stores it, then navigates to the error page and navigates to the stored page on 'try again'

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
