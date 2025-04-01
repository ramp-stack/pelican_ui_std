use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Column, Row, Padding, Offset, Size};
use crate::PelicanUI;


#[derive(Clone, Debug)]
pub struct ListItem(Row, Option<Icon>, Option<Avatar>, ListItemData);

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
        circle_icon: Option<AvatarContent>,
        on_click: fn(&mut Context, (u32, u32)) -> (),
    ) -> Self {
        ListItem (
            Row(16, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.clone().map(|data| Avatar::new(ctx, data, None, false, 48)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
        )
    }
}

#[derive(Clone, Debug)]
struct RadioButton;

impl RadioButton {
    pub fn new(ctx: &mut Context, is_enabled: bool) -> Icon {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio "};
        Icon::new(ctx, icon, color, 32)
    }
}

#[derive(Clone, Debug)]
struct ListItemData(pub Row, pub LeftData, pub Option<RightData>);

impl ListItemData {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
    ) -> Self {
        ListItemData (
            Row(8, Offset::Start, Size::Fit, Padding::default()),
            LeftData::new(ctx, title, flair, subtitle, description),
            right_title.map(|r_title| RightData::new(ctx, r_title, right_subtitle)), 
        )
    }
}
#[derive(Clone, Debug)]
struct TitleRow(Row, BasicText, Option<Icon>);

impl TitleRow {
    pub fn new(ctx: &mut Context, title: &'static str, flair: Option<(&'static str, Color)>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        TitleRow(
            Row(8, Offset::Start, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size),
            flair.map(|(name, color)| Icon::new(ctx, name, color, 20)),
        )
    }
}

#[derive(Clone, Debug)]
struct LeftData(pub Column, pub TitleRow, pub Option<BasicText>, pub Option<BasicText>);

impl LeftData {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.xs;
        LeftData (
            Column(4, Offset::Start, Size::Fit, Padding::default()),
            TitleRow::new(ctx, title, flair),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
            description.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
        )
    }
}

#[derive(Clone, Debug)]
struct RightData(Column, BasicText, Option<BasicText>);

impl RightData {
    pub fn new(ctx: &mut Context, title: &'static str, subtitle: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        RightData (
            Column(4, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size.h5),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
        )
    }
}

// pub enum AccountType { Spending, Savings }

impl ListItem {
    pub fn contact(
        ctx: &mut Context, 
        data: AvatarContent, 
        name: &'static str, 
        nym: &'static str, 
        on_click: fn(&mut Context, (u32, u32)) -> ()
    ) -> Self {
        ListItem::new(ctx, name, None, Some(nym), None, None, None, None, Some(data), on_click)
    }

    // pub fn new_direct_message(data: AvatarData, name: &'static str, recent: &'static str) -> Self {
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
    //         circle_icon: Some(AvatarData::Icon(Icon::Group, IconStyle::Secondary)),
    //         title: Some("Group Message"),
    //         title_flair: None,
    //         subtitle: None,
    //         description: Some(description),
    //         title_right: None,
    //         subtitle_right: None,
    //     }
    // } 

    // pub fn new_room(data: AvatarData, name: &'static str, members: &'static str, desc: &'static str) -> Self {
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
    //         circle_icon: Some(AvatarData::Icon(icon, IconStyle::Brand)),
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
