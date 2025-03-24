use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent, CircleIconStyle};
use crate::layout::{Row, RowOffset, Column, ColumnOffset, Stack, Offset, Size};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

pub struct ListItem(pub CircleIcon, pub CircleIcon); //pub Option<Icon>, pub Option<CircleIcon>, pub _ListItemData);

impl ListItem {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
        radio_button: Option<bool>,
        circle_icon: Option<CircleIconContent>,
        on_click: fn() -> (),
    ) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, font_size) = (theme.colors, theme.fonts.size);

        ListItem (
            radio_button.map(|enabled| _RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| CircleIcon::new(ctx, data, None, false, 48)),
            _ListItemData (
                _LeftData (
                    _TitleRow (
                        Text::new(ctx, title, TextStyle::Heading, font_size.h5),
                        flair.map(|(name, color)| Icon::new(ctx, name, color, 20)),
                    ),
                    subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
                    description.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
                ),
                right_title.map(|r_title| { _RightData (
                    Text::new(ctx, r_title, TextStyle::Heading, font_size.h5),
                    right_subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
                )}),
            )
        )
    }
}

impl Component for ListItem {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![];
        if let Some(radio_button) = &mut self.0 { children.push(radio_button); }
        if let Some(circle_icon) = &mut self.1 { children.push(circle_icon); }
        children.push(&mut self.2);
        Container::new(Row(16, RowOffset::Center), vec![&mut self.0, &mut self.1])
    }
}

struct _RadioButton;

impl _RadioButton {
    pub fn new(ctx: &mut Context, is_enabled: bool) -> Icon {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio "};
        Icon::new(ctx, icon, color, 32)
    }
}

struct _ListItemData(pub _LeftData, pub Option<_RightData>);

impl Component for _ListItemData {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![&mut self.0];
        if let Some(right_data) = &mut self.1 { children.push(right_data); }
        Container::new(Row(8, RowOffset::Top), children)
    }
}

struct _TitleRow(pub BasicText, pub Option<Icon>);

impl Component for _TitleRow {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![&mut self.0];
        if let Some(flair) = &mut self.1 { children.push(flair); }
        Container::new(Row(8, RowOffset::Center), children)
    }
}

struct _LeftData(pub _TitleRow, pub Option<BasicText>, pub Option<BasicText>);

impl Component for _LeftData {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![&mut self.0];
        if let Some(subtitle) = &mut self.1 { children.push(subtitle); }
        if let Some(description) = &mut self.2 { children.push(description); }
        Container::new(Column(4, ColumnOffset::Left), children)
    }
}

struct _RightData(pub BasicText, pub Option<BasicText>);

impl Component for _RightData {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![&mut self.0];
        if let Some(subtitle) = &mut self.1 { children.push(subtitle); }
        Container::new(Column(4, ColumnOffset::Right), children)
    }
}

// pub enum AccountType { Spending, Savings }

impl ListItem {
    pub fn contact(ctx: &mut Context, data: CircleIconContent, name: &'static str, nym: &'static str, on_click: fn() -> ()) -> Self {
        ListItem::new(ctx, name, None, Some(nym), None, None, None, None, Some(data), on_click)
    }

    // pub fn new_direct_message(data: CircleIconData, name: &'static str, recent: &'static str) -> Self {
    //     Self {
    //         radio: None,
    //         circle_icon: Some(data),
    //         title: Some(name),
    //         title_flair: None,
    //         subtitle: None,
    //         description: Some(recent),
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // }

    // pub fn new_group_message(names: Vec<&'static str>) -> Self {
    //     let description = Box::leak(names.join(", ").into_boxed_str());

    //     Self {
    //         radio: None,
    //         circle_icon: Some(CircleIconData::Icon(Icon::Group, IconStyle::Secondary)),
    //         title: Some("Group Message"),
    //         title_flair: None,
    //         subtitle: None,
    //         description: Some(description),
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // } 

    // pub fn new_room(data: CircleIconData, name: &'static str, members: &'static str, desc: &'static str) -> Self {
    //     Self {
    //         radio: None,
    //         circle_icon: Some(data),
    //         title: Some(name),
    //         title_flair: None,
    //         subtitle: Some(members),
    //         description: Some(desc),
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // } 

    // pub fn new_bitcoin(is_received: bool, usd: &'static str, date: &'static str) -> Self {
    //     let title = is_received { "Received Bitcoin" } else { "Sent Bitcoin" };
    //     Self {
    //         radio: None,
    //         circle_icon: None,
    //         title: Some(title),
    //         title_flair: None,
    //         subtitle: Some(date),
    //         description: None,
    //         title_right: Some(usd),
    //         subtitle_right: Some("Details"),
    //     }
    // }

    // pub fn new_bitcoin_sending(usd: &'static str, btc: &'static str, date: &'static str) -> Self {
    //     Self {
    //         radio: None,
    //         circle_icon: None,
    //         title: Some("Sending Bitcoin"),
    //         title_flair: Some((Icon::Warning, COLORS.status.warning)),
    //         subtitle: Some(date),
    //         description: None,
    //         title_right: Some(usd),
    //         subtitle_right: Some(btc),
    //     }
    // }

    // pub fn new_bitcoin_account(variant: AccountType, usd: &'static str, btc: &'static str) -> Self {
    //     let (icon, subtitle) = match variant {
    //         AccountType::Savings => (Icon::Safe, "Savings"),
    //         AccountType::Spending => (Icon::Wallet, "Spending")
    //     };

    //     Self {
    //         radio: None,
    //         circle_icon: Some(CircleIconData::Icon(icon, IconStyle::Brand)),
    //         title: Some("Wallet"),
    //         title_flair: None,
    //         subtitle: Some(subtitle),
    //         description: None,
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // }

    // pub fn new_selection(selected: bool, title: &'static str, subtitle: &'static str) -> Self {
    //     Self {
    //         radio: Some(selected),
    //         circle_icon: None,
    //         title: Some(title),
    //         title_flair: None,
    //         subtitle: Some(subtitle),
    //         description: None,
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // }
}

// pub enum ListItem {
//     Contact(RgbaImage, &'static str, &'static str), // Image, Name, Nym
//     DirectMessage(RgbaImage, &'static str, &'static str), // Image, Name, Recent Message
//     GroupMessage(Vec<&'static str>), // Members Names
//     Room(RgbaImage, &'static str, &'static str, &'static str), // Image, Name, Members, Description
//     Bitcoin(bool, &'static str, &'static str), // IsReceived, USD, Date
//     SendingBitcoin(&'static str, &'static str, &'static str), // USD, BTC, Date
//     BitcoinAccount(AccountType, &'static str, &'static str), // Account Type, USD, BTC
//     Selection(bool, &'static str, &'static str), // Selected, Title, Subtitle
// }
