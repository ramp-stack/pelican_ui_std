pub mod components;
pub mod theme;
pub mod layout;
pub mod elements;

use rust_on_rails::prelude::*;

use crate::theme::Theme;

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

impl Plugin for PelicanUI {
    fn name() -> &'static str { "Pelican UI" }
}

pub mod prelude {
    pub use crate::components::button::{Button};
    // pub use crate::theme::colors::*;
    pub use crate::elements::icon::Icon;
    // pub use crate::theme::fonts::{Text, TextSize, TextStyle};
    pub use crate::{Column, Row};
    // pub use crate::*;
    // pub use rust_on_rails::prelude::*;
    pub use crate::elements::shapes::Circle;
    // pub use crate::elements::image::Image as ProfileImage;
    // pub use crate::ZERO;
    pub use crate::layout::{Column, Row, Padding};
    pub use crate::elements::text::{Text, TextStyle};
    pub use crate::components::circle_icon::{CircleIcon, CircleIconData, CircleIconStyle};
    pub use crate::theme::Theme;
    pub use crate::PelicanUI;
}

pub mod custom {
	pub use crate::theme::colors::{
	    ColorResources,
	    BackgroundColor,
	    OutlineColor,
	    StatusColor,
	    TextColor,
	    BrandColor,
	    ShadesColor
	};

	pub use crate::theme::fonts::{
	    FontResources,
	    Fonts,
	    FontSize,
	};
}
