#![allow(unused)]
use rust_on_rails::prelude::*;
use std::collections::HashMap;

use crate::components::button::{ButtonState, ButtonStyle};

#[derive(Clone)]
pub struct ColorResources { 
    pub background: BackgroundColor, 
    pub outline: OutlineColor, 
    pub status: StatusColor,
    pub text: TextColor,
    pub button: ButtonColors,
    pub brand: BrandColor,
    pub shades: ShadesColor,
}

impl Default for ColorResources {
    fn default() -> Self {
        ColorResources {
            background: BackgroundColor::default(),
            outline: OutlineColor::default(),
            status: StatusColor::default(),
            text: TextColor::default(),
            brand: BrandColor::default(),
            shades: ShadesColor::default(),
            button: ButtonColors::new(
                ButtonSchemes::default(),
            ),
        } 
    }
}

impl ColorResources {
    pub fn new(
        background: BackgroundColor,
        outline: OutlineColor,
        status: StatusColor,
        text: TextColor,
        brand: BrandColor,
        shades: ShadesColor,
        button: ButtonColors,
    ) -> Self {
        ColorResources { background, outline, status, text, brand, shades, button }
    }
}

#[derive(Copy, Clone)]
pub struct ShadesColor {
    pub black: Color,
    pub white: Color,
    pub lighten: Color,
    pub lighten2: Color,
    pub darken: Color,
    pub darken2: Color
}

impl Default for ShadesColor {
    fn default() -> Self {
        ShadesColor {
            black: Color::from_hex("000000", 255),
            white: Color::from_hex("ffffff", 255),
            lighten: Color::from_hex("ffffff", 20),
            lighten2: Color::from_hex("ffffff", 40),
            darken: Color::from_hex("000000", 20),
            darken2: Color::from_hex("000000", 40)
        }
    }
}

#[derive(Copy, Clone)]
pub struct BackgroundColor {
    pub primary: Color,
    pub secondary: Color
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor {
            primary: Color::from_hex("000000", 255),
            secondary: Color::from_hex("262322", 255),
        }
    }
}

#[derive(Copy, Clone)]
pub struct BrandColor {
    pub primary: Color,
    pub secondary: Color
}

impl Default for BrandColor {
    fn default() -> Self {
        BrandColor {
            primary: Color::from_hex("eb343a", 255),
            secondary: Color::from_hex("ffffff", 255)
        }
    }
}


#[derive(Copy, Clone)]
pub struct OutlineColor {
    pub primary: Color,
    pub secondary: Color
}

impl Default for OutlineColor {
    fn default() -> Self {
        OutlineColor {
            primary: Color::from_hex("ffffff", 255),
            secondary: Color::from_hex("585250", 255),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextColor {
    pub heading: Color,
    pub primary: Color,
    pub secondary: Color,
    pub danger: Color
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor{
            heading: Color::from_hex("ffffff", 255),
            primary: Color::from_hex("e2e1df", 255),
            secondary: Color::from_hex("a7a29d", 255),
            danger: Color::from_hex("ff330a", 255)
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatusColor {
    pub success: Color,
    pub warning: Color,
    pub danger: Color
}

impl Default for StatusColor {
    fn default() -> Self {
        StatusColor{
            success: Color::from_hex("3ccb5a", 255),
            warning: Color::from_hex("f5bd14", 255),
            danger: Color::from_hex("ff330a", 255),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ButtonColorScheme {
    pub background: Color,
    pub label: Color,
    pub outline: Color,
}

#[derive(Copy, Clone)]
pub struct ButtonSchemes {
    pub primary_default: ButtonColorScheme,
    pub primary_disabled: ButtonColorScheme,
    pub primary_hover: ButtonColorScheme,
    pub primary_selected: ButtonColorScheme,

    pub secondary_default: ButtonColorScheme,
    pub secondary_disabled: ButtonColorScheme,
    pub secondary_hover: ButtonColorScheme,
    pub secondary_selected: ButtonColorScheme,

    pub ghost_default: ButtonColorScheme,
    pub ghost_disabled: ButtonColorScheme,
    pub ghost_hover: ButtonColorScheme,
    pub ghost_selected: ButtonColorScheme,
}

impl Default for ButtonSchemes {
    fn default() -> Self {
        ButtonSchemes {
            primary_default: ButtonColorScheme {
                background: Color::from_hex("eb343a", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_disabled: ButtonColorScheme {
                background: Color::from_hex("443f3f", 255),
                label: Color::from_hex("000000", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_hover: ButtonColorScheme {
                background: Color::from_hex("da282e", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_selected: ButtonColorScheme {
                background: Color::from_hex("b71e23", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },

            secondary_default: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },
            secondary_disabled: ButtonColorScheme {
                background: Color::from_hex("78716c", 255),
                label: Color::from_hex("000000", 255),
                outline:Color::from_hex("585250", 255),
            },
            secondary_hover: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },
            secondary_selected: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },

            ghost_default: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_disabled: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("78716c", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_hover: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_selected: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct ButtonColors {
    color_map: HashMap<(ButtonState, ButtonStyle), ButtonColorScheme>,
}

impl ButtonColors {
    pub fn new(schemes: ButtonSchemes) -> Self {
        let mut color_map = HashMap::new();

        color_map.insert((ButtonState::Default, ButtonStyle::Primary), schemes.primary_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Primary), schemes.primary_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Primary), schemes.primary_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Primary), schemes.primary_selected);

        color_map.insert((ButtonState::Default, ButtonStyle::Secondary), schemes.secondary_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Secondary), schemes.secondary_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Secondary), schemes.secondary_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Secondary), schemes.secondary_selected);

        color_map.insert((ButtonState::Default, ButtonStyle::Ghost), schemes.ghost_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Ghost), schemes.ghost_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Ghost), schemes.ghost_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Ghost), schemes.ghost_selected);

        ButtonColors{ color_map }
    }

    pub fn colors_from(&self, style: ButtonStyle, state: ButtonState) -> ButtonColorScheme {
        self.color_map.get(&(state, style)).copied().expect("ColorScheme Not Found")
    }
}
