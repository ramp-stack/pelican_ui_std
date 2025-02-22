#![allow(unused)]
use std::collections::HashMap;
use crate::components::inputs::{ButtonState, ButtonStyle};

#[derive(Clone)]
pub struct ColorResources { 
    pub background: BackgroundColor, 
    pub outline: OutlineColor, 
    pub status: StatusColor,
    pub text: TextColor,
    pub button: ButtonColors,
    pub brand: BrandColor,
}

impl Default for ColorResources {
    fn default() -> Self {
        ColorResources {
            background: BackgroundColor::default(),
            outline: OutlineColor::default(),
            status: StatusColor::default(),
            text: TextColor::default(),
            brand: BrandColor::default(),
            button: ButtonColors::new(
                ButtonSchemes::default(),
            ),
        } 
    }
}

impl ColorResources {
    fn new(
        background: BackgroundColor,
        outline: OutlineColor,
        status: StatusColor,
        text: TextColor,
        brand: BrandColor,
        button: ButtonColors,
    ) -> Self {
        ColorResources { background, outline, status, text, brand, button }
    }
}

#[derive(Copy, Clone)]
pub struct BackgroundColor {
    pub primary: &'static str,
    pub secondary: &'static str,
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor {
            primary: "000000",
            secondary: "262322",
        }
    }
}

#[derive(Copy, Clone)]
pub struct BrandColor {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub text: &'static str,
}

impl Default for BrandColor {
    fn default() -> Self {
        BrandColor {
            primary: "3598fc",
            secondary: "3598fc",
            text: "ffffff"
        }
    }
}


#[derive(Copy, Clone)]
pub struct OutlineColor {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub tint: &'static str,
}

impl Default for OutlineColor {
    fn default() -> Self {
        OutlineColor {
            primary: "ffffff",
            secondary: "585250",
            tint: "585250",
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextColor {
    pub heading: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor{
            heading: "ffffff",
            primary: "e2e1df",
            secondary: "a7a29d",
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatusColor {
    pub success: &'static str,
    pub warning: &'static str,
    pub danger: &'static str,
}

impl Default for StatusColor {
    fn default() -> Self {
        StatusColor{
            success: "3ccb5a",
            warning: "f5bd14",
            danger: "eb343a",
        }
    }
}

#[derive(Copy, Clone)]
pub struct ButtonColorScheme {
    pub background: &'static str,
    pub label: &'static str,
    pub outline: &'static str,
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
                background: "3598fc",
                label: "ffffff",
                outline: transparent(),
            },
            primary_disabled: ButtonColorScheme {
                background: "443f3f",
                label: "000000",
                outline: transparent(),
            },
            primary_hover: ButtonColorScheme {
                background: "da282e",
                label: "ffffff",
                outline: transparent(),
            },
            primary_selected: ButtonColorScheme {
                background: "b71e23",
                label: "ffffff",
                outline: transparent(),
            },

            secondary_default: ButtonColorScheme {
                background: transparent(),
                label: "ffffff",
                outline: "585250",
            },
            secondary_disabled: ButtonColorScheme {
                background: "78716c",
                label: "000000",
                outline: "585250",
            },
            secondary_hover: ButtonColorScheme {
                background: "262322",
                label: "ffffff",
                outline: "585250",
            },
            secondary_selected: ButtonColorScheme {
                background: transparent(),
                label: "ffffff",
                outline: "585250",
            },

            ghost_default: ButtonColorScheme {
                background: transparent(),
                label: "ffffff",
                outline: transparent(),
            },
            ghost_disabled: ButtonColorScheme {
                background: transparent(),
                label: "78716c",
                outline: transparent(),
            },
            ghost_hover: ButtonColorScheme {
                background: "262322",
                label: "ffffff",
                outline: transparent(),
            },
            ghost_selected: ButtonColorScheme {
                background: "262322",
                label: "ffffff",
                outline: transparent(),
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

// TEMP
pub fn transparent() -> &'static str {
    "000000"
}
