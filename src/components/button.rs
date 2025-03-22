use crate::elements::icon::Icon;
use crate::elements::shapes::RoundedRectangle;
use crate::elements::text::{TextStyle, Text};

use crate::components::circle_icon::CircleIcon;

use crate::layout::Row;

use crate::discard_nonsies;
use crate::PelicanUI;

use rust_on_rails::prelude::*;

#[derive(Clone)]
pub struct Button {
    label: &'static str,
    size: ButtonSize,
    width: ButtonWidth,
    style: ButtonStyle,
    photo: Option<resources::Image>,
    icon_l: Option<&'static str>,
    icon_r: Option<&'static str>
}

impl Component for Button {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let theme = &ctx.get::<PelicanUI>().theme;
        let colors = theme.colors.button.colors_from(self.style, ButtonState::Default);

        let (text_size, height, icon_size, x_padding, spacing) = match self.size {
            ButtonSize::Medium => (theme.fonts.size.md, 32, 16, 12, 4),
            ButtonSize::Large => (theme.fonts.size.lg, 48, 24, 24, 12)
        };

        let size = match self.width {
            ButtonWidth::Hug => Size::Fit,
            ButtonWidth::Expand => Size::Static(max_size.0, height),
        };
        
        Container(Offset::Center, size, vec![Box::new(
            Container(
                Offset::Center, 
                Size::Static(max_size.0, height),
                vec![
                    Box::new(RoundedRectangle(colors.background, colors.outline, 1, height / 2)),
                    Box::new(Row(spacing, Offset::Center, discard_nonsies![
                        match &self.photo {
                            Some(image) => Some(Box::new(CircleIcon::Image(image.clone(), None, false, icon_size))),
                            None => None,
                        },
                        match self.icon_l {
                            Some(icon) => Some(Box::new(Icon(icon, colors.label, icon_size))),
                            None => None,
                        },
                        Some(Box::new(Text(TextStyle::Label(colors.label), self.label, text_size))),
                        match self.icon_r {
                            Some(icon) => Some(Box::new(Icon(icon, colors.label, icon_size))),
                            None => None,
                        },
                    ]))
                ]
            )
        )])
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Large,
    Medium,
}

impl Button {
    pub fn primary(label: &'static str) -> Self {
        Self {
            label,
            size: ButtonSize::Large,
            width: ButtonWidth::Expand,
            style: ButtonStyle::Primary,
            photo: None,
            icon_l: None,
            icon_r: None
        }
    }

    pub fn secondary(
        icon_l: Option<&'static str>, 
        label: &'static str, 
        icon_r: Option<&'static str>
    ) -> Self {
        Self {
            label,
            size: ButtonSize::Medium,
            width: ButtonWidth::Hug,
            style: ButtonStyle::Secondary,
            photo: None,
            icon_l,
            icon_r,
        }
    }

    pub fn ghost(label: &'static str) -> Self {
        Self {
            label,
            size: ButtonSize::Medium,
            width: ButtonWidth::Hug,
            style: ButtonStyle::Ghost,
            photo: None,
            icon_l: None,
            icon_r: None
        }
    }

    pub fn keypad(label: &'static str, icon_l: Option<&'static str>) -> Self {
        Self {
            label,
            size: ButtonSize::Large,
            width: ButtonWidth::Expand,
            style: ButtonStyle::Ghost,
            photo: None,
            icon_l,
            icon_r: None
        }
    }

    pub fn photo(
        label: &'static str, 
        photo: resources::Image,
    ) -> Self {
        Self {
            label,
            size: ButtonSize::Medium,
            width: ButtonWidth::Hug,
            style: ButtonStyle::Secondary,
            photo: Some(photo),
            icon_l: None,
            icon_r: None
        }
    }

    // pub fn button_row(a: &'static str, b: &'static str) -> Row {
    //     Row(ZERO, 16, Align::Center, vec![Self::primary(a), Self::primary(b)])
    // }

    // pub fn quick_actions(colorss: Vec<(Icon, &'static str)>) -> Wrap {
    //     let children = colorss
    //         .into_iter()
    //         .map(|colors| {
    //             Self::secondary(colors.1, Some(colors.1), None)
    //         }).collect();

    //     Wrap(ZERO, 8, Align::Left, children)
    // }

    // pub fn quick_deselect(colorss: Vec<&'static str>) -> Wrap {
    //     let children = colorss
    //         .into_iter()
    //         .map(|label| {
    //             Self::secondary(label, None, Some(Icon::Close))
    //         }).collect();

    //     Wrap(ZERO, 8, Align::Left, children)
    // }
}