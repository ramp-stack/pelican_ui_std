use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent, CircleIconStyle};
use crate::layout::{Row, Column, Stack, Offset, Size};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

#[derive(Clone, Debug)]
pub struct ListItem(pub Row, pub Option<Icon>, pub Option<CircleIcon>, pub Option<CircleIcon>, pub _ListItemData);

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
        ListItem (
            Row(16, Offset::Center, Size::Fill),
            radio_button.map(|enabled| _RadioButton::new(ctx, enabled)), 
            circle_icon.clone().map(|data| CircleIcon::new(ctx, data, None, false, 48)),
            _ListItemData (
                Row(8, Offset::Start, Size::Fit),
                _LeftData::new(title, flair, subtitle, description),
                right_title.map(|r_title| _RightData::new(r_title, right_subtitle)), 
            )
        )
    }
}

impl Component for ListItem {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![];
        if let Some(radio_button) = &mut self.1 { children.push(radio_button); }
        if let Some(circle_icon) = &mut self.2 { children.push(circle_icon); }
        children.push(&mut self.4);
        children
    }
    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![];
        if let Some(radio_button) = &self.1 { children.push(radio_button); }
        if let Some(circle_icon) = &self.2 { children.push(circle_icon); }    
        children.push(&self.4);
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

#[derive(Clone, Debug)]
struct _RadioButton;

impl _RadioButton {
    pub fn new(ctx: &mut Context, is_enabled: bool) -> Icon {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio "};
        Icon::new(ctx, icon, color, 32)
    }
}

#[derive(Clone, Debug)]
struct _ListItemData(pub Row, pub _LeftData, pub Option<_RightData>);

impl Component for _ListItemData {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&mut self.1];
        if let Some(right_data) = &mut self.2 { children.push(right_data); }
        children
    }
    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![&self.1];
        if let Some(right_data) = &self.2 { children.push(right_data); }
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

#[derive(Clone, Debug)]
struct _TitleRow(pub Row, pub BasicText, pub Option<Icon>);

impl _TitleRow {
    pub fn new(title: &'static str, flair: Option<(&'static str, Color)>) -> Self {
        let font_size = &ctx.get::<PelicanUI>().theme.fonts.size.h5;
        _TitleRow(
            Row(8, Offset::Start, Size::Fit),
            Text::new(ctx, title, TextStyle::Heading, font_size),
            flair.map(|(name, color)| Icon::new(ctx, name, color, 20)),
        )
    }
}

impl Component for _TitleRow {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&mut self.1];
        if let Some(flair) = &mut self.2 { children.push(flair); }
        children
    }
    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![&self.1];
        if let Some(flair) = &self.2 { children.push(flair); }
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

#[derive(Clone, Debug)]
struct _LeftData(pub Column, pub _TitleRow, pub Option<BasicText>, pub Option<BasicText>);

impl _LeftData {
    pub fn new(
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
    ) -> Self {
        let font_size = &ctx.get::<PelicanUI>().theme.fonts.size.xs;
        _LeftData (
            Column(4, Offset::Start, Size::Fit),
            _TitleRow::new(title, flair),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
            description.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
        )
    }
}

impl Component for _LeftData {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&mut self.1];
        if let Some(subtitle) = &mut self.2 { children.push(subtitle); }
        if let Some(description) = &mut self.3 { children.push(description); }
        children
    }
    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![&self.1];
        if let Some(subtitle) = &self.2 { children.push(subtitle); }
        if let Some(description) = &self.3 { children.push(description); }
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

#[derive(Clone, Debug)]
struct _RightData(pub Column, pub BasicText, pub Option<BasicText>);

impl _RightData {
    pub fn new(title: &'static str, subtitle: Option<&'static str>) -> Self {
        let font_size = &ctx.get::<PelicanUI>().theme.fonts.size;
        _RightData (
            Column(4, Offset::End, Size::Fit),
            Text::new(ctx, title, TextStyle::Heading, font_size.h5),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
        ),
    }
}

impl Component for _RightData {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&mut self.1];
        if let Some(subtitle) = &mut self.2 { children.push(subtitle); }
        children
    }
    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![&self.1];
        if let Some(subtitle) = &self.2 { children.push(subtitle); }
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
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
