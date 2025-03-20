use rust_on_rails::prelude::*;
use crate::PelicanUI;

pub enum TextStyle {
    Heading,
    Primary,
    Secondary,
    Error,
    White,
    Label,
}

pub struct Text(pub TextStyle, pub &'static str, pub u32);

impl ComponentBuilder for Text {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, fonts) = (theme.colors, theme.fonts.fonts.clone());

        let (color, font) = match self.0 {
            TextStyle::Heading => (colors.text.heading, fonts.heading.clone()),
            TextStyle::Primary => (colors.text.primary, fonts.text.clone()),
            TextStyle::Secondary => (colors.text.secondary, fonts.text.clone()),
            TextStyle::Error => (colors.status.danger, fonts.text.clone()),
            TextStyle::White => (colors.text.heading, fonts.text.clone()),
            TextStyle::Label => (colors.text.heading, fonts.label.clone())
        };
        
        // let children = self.1.chars().map(|ch| {
        //     match is_emoji(ch) {
        //         true => BasicText(&ch.to_string(), color, None, self.2, (self.2 as f32*1.25) as u32, fonts.emoji.clone()),
        //         false => BasicText(&ch.to_string(), color, None, self.2, (self.2 as f32*1.25) as u32, font)
        //     }
        // })
        
        // Row!(ZERO, 4, Align::Left, children).build_children(ctx, max_size)

        BasicText(self.1, color, None, self.2, (self.2 as f32*1.25) as u32, font)
    }
}

// println!(
//     "EMOJI COUNT: {:?}", 
//     self.0.chars()
//     .filter(|&ch| is_emoji(ch))
//     .collect::<Vec<char>>()
// );

// fn is_emoji(ch: char) -> bool {
//     matches!(
//         ch as u32,
//         0x1F600..=0x1F64F | // Emoticons
//         0x1F300..=0x1F5FF | // Misc Symbols and Pictographs
//         0x1F680..=0x1F6FF | // Transport and Map
//         0x2600..=0x26FF   | // Misc symbols
//         0x2700..=0x27BF   | // Dingbats
//         0xFE00..=0xFE0F   | // Variation Selectors
//         0x1F900..=0x1F9FF | // Supplemental Symbols and Pictographs
//         0x1F1E6..=0x1F1FF   // Flags
//     )
// }