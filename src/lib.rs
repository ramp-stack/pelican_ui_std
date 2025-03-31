pub mod theme;
pub mod layout;
pub mod elements;
pub mod components;

use rust_on_rails::prelude::*;

use crate::theme::Theme;

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
    pub use crate::elements::images::{Icon, CircleIconStyle};
    // pub use crate::elements::text::{Text, TextStyle};
    pub use crate::components::avatar::{Avatar, AvatarContent};
    // pub use crate::components::list_item::ListItem;
    pub use crate::components::button::{Button, ButtonStyle, ButtonWidth, ButtonSize, ButtonState};
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
