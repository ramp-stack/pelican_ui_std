use pelican_ui::drawable::Color;
use pelican_ui::{ColorResources, ButtonColors, ButtonColorScheme, BackgroundColor, TextColor, BrandColor, IllustrationColors};
pub struct PelicanColorPelicanThemes;




impl ColorResources {
    
}

#[derive(Copy, Clone, Debug)]
pub enum PelicanTheme {
    Light, Dark, HighContrast, Sunset, Oceanic, Forest, Cyberpunk, MinimalistGray
}

impl PelicanTheme {
    pub fn colors(&self) -> ColorResources {
        match self {
            PelicanTheme::Light => {
                let brand = Color::from_hex("0078D4", 255);
                ColorResources::new(
                    BackgroundColor { primary: Color::from_hex("FFFFFF", 255), secondary: Color::from_hex("F5F5F5", 255) },
                    OutlineColor { primary: Color::from_hex("000000", 255), secondary: Color::from_hex("585250", 255) },
                    StatusColor { success: Color::from_hex("3CCB5A", 255), warning: Color::from_hex("F5BD14", 255), danger: Color::from_hex("FF330A", 255) },
                    TextColor { heading: Color::from_hex("000000", 255), primary: Color::from_hex("333333", 255), secondary: Color::from_hex("777777", 255) },
                    BrandColor { primary: brand, secondary: Color::from_hex("FFFFFF", 255) },
                    Self::button_from_brand(brand, false),
                    IllustrationColors::default(),
                )
            }
            PelicanTheme::Dark => {
                let brand = Color::from_hex("0A84FF", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("121212", 255), secondary: Color::from_hex("1E1E1E", 255) },
                    outline: OutlineColor { primary: Color::from_hex("FFFFFF", 255), secondary: Color::from_hex("585250", 255) },
                    status: StatusColor { success: Color::from_hex("3CCB5A", 255), warning: Color::from_hex("F5BD14", 255), danger: Color::from_hex("FF330A", 255) },
                    text: TextColor { heading: Color::from_hex("FFFFFF", 255), primary: Color::from_hex("E2E1DF", 255), secondary: Color::from_hex("A7A29D", 255) },
                    brand: BrandColor { primary: brand, secondary: Color::from_hex("FFFFFF", 255) },
                    button: Self::button_from_brand(brand, true),
                    illustration: IllustrationColors::default(),
                }
            }
            PelicanTheme::HighContrast => {
                let brand = Color::from_hex("FFD700", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("000000", 255), secondary: Color::from_hex("1A1A1A", 255) },
                    outline: OutlineColor { primary: Color::from_hex("FFFFFF", 255), secondary: Color::from_hex("FFD700", 255) },
                    status: StatusColor { success: Color::from_hex("00FF00", 255), warning: Color::from_hex("FFFF00", 255), danger: Color::from_hex("FF0000", 255) },
                    text: TextColor { heading: Color::from_hex("FFFFFF", 255), primary: Color::from_hex("FFFFFF", 255), secondary: Color::from_hex("FFD700", 255) },
                    brand: BrandColor { primary: brand, secondary: Color::from_hex("FFFFFF", 255) },
                    button: Self::button_from_brand(brand, true),
                    illustration: IllustrationColors::default(),
                }
            }
            PelicanTheme::Sunset => {
                let brand = Color::from_hex("FF6F3C", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("FFF5E6", 255), secondary: Color::from_hex("FFE8D6", 255) },
                    ..Default::default()
                }
            }
            PelicanTheme::Oceanic => {
                let brand = Color::from_hex("00A3A3", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("E0F7FA", 255), secondary: Color::from_hex("B2EBF2", 255) },
                    ..Default::default()
                }
            }
            PelicanTheme::Forest => {
                let brand = Color::from_hex("228B22", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("F0F4F0", 255), secondary: Color::from_hex("DFF0DF", 255) },
                    ..Default::default()
                }
            }
            PelicanTheme::Cyberpunk => {
                let brand = Color::from_hex("FF00FF", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("0A0A0A", 255), secondary: Color::from_hex("1A1A1A", 255) },
                    ..Default::default()
                }
            }
            PelicanTheme::MinimalistGray => {
                let brand = Color::from_hex("607D8B", 255);
                ColorResources {
                    background: BackgroundColor { primary: Color::from_hex("F5F5F5", 255), secondary: Color::from_hex("E0E0E0", 255) },
                    ..Default::default()
                }
            }
        }
    }

    fn button_from_brand(brand: Color, dark_theme: bool) -> ButtonColors {
        let primary_hover = brand.darken(0.1).saturate(0.05);
        let primary_pressed = brand.darken(0.2).saturate(0.1);
        let label_color = if dark_theme { Color::from_hex("FFFFFF", 255) } else { Color::from_hex("FFFFFF", 255) };

        ButtonColors {
            primary_default: ButtonColorScheme { background: brand, label: label_color, outline: Color::from_hex("000000", 0) },
            primary_disabled: ButtonColorScheme { background: Self::darken(color, 0.6), label: label_color.darken(0.5), outline: Color::from_hex("000000", 0) },
            primary_hover: ButtonColorScheme { background: primary_hover, label: label_color, outline: Color::from_hex("000000", 0) },
            primary_selected: ButtonColorScheme { background: primary_hover, label: label_color, outline: Color::from_hex("000000", 0) },
            primary_pressed: ButtonColorScheme { background: primary_pressed, label: label_color, outline: Color::from_hex("000000", 0) },
            secondary_default: ButtonColorScheme { background: Color::from_hex("000000", 0), label: brand, outline: brand },
            secondary_disabled: ButtonColorScheme { background: Color::from_hex("000000", 0), label: Self::darken(brand, 0.5), outline: Self::darken(brand, 0.5) },
            secondary_hover: ButtonColorScheme { background: Self::darken(brand, 0.1), label: brand, outline: brand },
            secondary_selected: ButtonColorScheme { background: Self::darken(barnd, 0.2), label: brand, outline: brand },
            secondary_pressed: ButtonColorScheme { background: Self::.darken(brand, 0.3), label: brand, outline: brand },
            ghost_default: ButtonColorScheme { background: Color::from_hex("000000", 0), label: brand, outline: Color::from_hex("000000", 0) },
            ghost_disabled: ButtonColorScheme { background: Color::from_hex("000000", 0), label: brand.darken(0.5), outline: Color::from_hex("000000", 0) },
            ghost_hover: ButtonColorScheme { background: Self::darken(brand, 0.1), label: brand, outline: Color::from_hex("000000", 0) },
            ghost_selected: ButtonColorScheme { background: Self::darken(brand, 0.2), label: brand, outline: Color::from_hex("000000", 0) },
            ghost_pressed: ButtonColorScheme { background: Self::darken(brand, 0.3), label: brand, outline: Color::from_hex("000000", 0) },
        }
    }

    fn darken(color: Color, pct: f32) -> Color {
        let r = ((color.0 as f32 * (1.0 - pct)).clamp(0.0, 255.0)) as u8,
        let g = ((color.1 as f32 * (1.0 - pct)).clamp(0.0, 255.0)) as u8,
        let b = ((color.2 as f32 * (1.0 - pct)).clamp(0.0, 255.0)) as u8,
        Color(r, g, b, color.3)
    }

    fn saturate(color: Color, pct: f32) -> Color {
        let r = ((color.0 as f32 + (color.0 as f32 - 128.0) * pct).clamp(0.0, 255.0)) as u8;
        let g = ((color.1 as f32 + (color.1 as f32 - 128.0) * pct).clamp(0.0, 255.0)) as u8;
        let b = ((color.2 as f32 + (color.2 as f32 - 128.0) * pct).clamp(0.0, 255.0)) as u8;
        
        Color(r, g, b, color.3)
    }
}