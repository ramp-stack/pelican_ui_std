pub mod events;
pub mod config;
pub mod theme;
pub mod layout;
pub mod elements;
pub mod components;
pub mod interface;

use rust_on_rails::prelude::*;
use crate::theme::Theme;

#[cfg(target_os = "ios")]
extern "C" {
    fn trigger_haptic();
    // fn get_application_support_dir() -> *const std::os::raw::c_char;
}
#[cfg(target_os = "ios")]
fn vibrate()  {
    unsafe {
        trigger_haptic();
    }
}

pub struct PelicanUI {
    pub theme: Theme,
}

impl PelicanUI {
    pub fn init(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

impl Plugin for PelicanUI {
    async fn background_tasks(_ctx: &mut HeadlessContext) -> Tasks {vec![]}

    async fn new(ctx: &mut Context, _h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        ctx.include_assets(include_assets!("./resources"));
        (PelicanUI{theme: Theme::default(ctx)}, vec![])
    }
}

pub trait AppPage: Drawable + std::fmt::Debug + 'static {}

pub trait AppFlow: std::fmt::Debug + Send + Sync + dyn_clone::DynClone + 'static {
    fn get_page(&self, ctx: &mut Context) -> Box<dyn crate::AppPage>;

    fn navigate(self, ctx: &mut Context) where Self: Sized {
        ctx.trigger_event(crate::events::NavigateEvent(Box::new(self) as Box<dyn AppFlow>));
    }
}

dyn_clone::clone_trait_object!(AppFlow);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementID(uuid::Uuid);

impl ElementID {
    pub fn new() -> Self {
        ElementID(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

pub mod prelude {
    pub use crate::ElementID;
    pub use crate::AppFlow;
    pub use crate::AppPage;
    pub use crate::events::*;
    pub use crate::interface::*;
    pub use crate::layout::*;
    pub use crate::components::*;
    pub use crate::elements::*;
    pub use crate::theme::Theme;
    pub use crate::PelicanUI;
}
