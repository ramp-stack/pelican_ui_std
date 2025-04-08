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

#[derive(Plugin)]
pub struct PelicanUI {
    pub theme: Theme,
}

impl PelicanUI {
    pub fn init(ctx: &mut Context) -> PelicanUI {
        ctx.include_assets(include_assets!("./resources"));
        PelicanUI { theme: Theme::default(ctx) }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

pub mod prelude {
    pub use crate::interface::*;
    pub use crate::layout::*;
    pub use crate::components::*;
    //pub use crate::elements::images::Icon;
    pub use crate::elements::text::{Text, TextStyle};
    pub use crate::theme::Theme;
    pub use crate::PelicanUI;
}

pub mod custom {
	// pub use crate::theme::colors::{
	//     ColorResources,
	//     BackgroundColor,
	//     OutlineColor,
	//     StatusColor,
	//     TextColor,
	//     BrandColor,
	//     ShadesColor
	// };

	// pub use crate::theme::fonts::{
	//     FontResources,
	//     Fonts,
	//     FontSize,
	// };
}


