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
