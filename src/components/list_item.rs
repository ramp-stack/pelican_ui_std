use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Row, Column, ZERO, Align };
use crate::components::{UserIcon, CircleIcon};

pub struct ListItem {
    radio: Option<bool>,
    circle_icon: Option<CircleIconData>,
    title: Option<&'static str>,
    title_flair: Option<(Icon, &'static str)>,
    subtitle: Option<&'static str>,
    description: Option<&'static str>,
    title_right: Option<&'static str>,
    subtitle_right: Option<&'static str>,
}

impl ComponentBuilder for ListItem {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut list_item: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut title_row: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut details: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut details_left: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut details_right: Vec<Box<dyn ComponentBuilder>> = vec![];
        
        if let Some(status) = &self.radio {
            list_item.push(RadioButton(status, 32)); // Radio Button
        }

        if let Some(circle_icon) = &self.circle_icon {
            list_item.push(CircleIcon(circle_icon, None, false, 48)); // Circle Icon
        }

        if let Some(title) = &self.title { 
            title_row.push(Text::heading(ctx, title, TextSize::h5())); // Title
        }

        if let Some((flair, color)) = &self.title_flair { 
            title_row.push(flair.build(20, color)); // Title Flair
        }

        details_left.push(Row(ZERO, 8, Align::Left, title_row));

        if let Some(subtitle) = &self.subtitle {
            details_left.push(Text::secondary(ctx, subtitle, TextSize::xs())); // Subtitle
        }

        if let Some(description) = &self.description {
            details_left.push(Text::secondary(ctx, subtitle, TextSize::xs())); // Description
        }

        if let Some(title) = &self.title_right {
            details_left.push(Text::heading(ctx, title, TextSize::h5())); // Title Right
        }

        if let Some(subtitle) = &self.subtitle_right {
            details_left.push(Text::secondary(ctx, subtitle, TextSize::xs()).underline()); // Subtitle Right
        }

        details.push(Column(ZERO, 4, Align::Left, details_left));
        details.push(Column(ZERO, 4, Align::Right, details_right));
        list_item.push(Row(ZERO, 8, Align::Left, details));

        Row(ZERO, 16, Align::Center, list_item).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


struct RadioButton(bool, u32);

impl ComponentBuilder for RadioButton {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let icon = if self.0 { Icon::RadioFilled } else { Icon::Radio };
        icon.build(self.1, COLORS.text.heading).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


// pub enum AccountType { Spending, Savings }

impl ListItem {
    pub fn new_contact(data: CircleIconData, name: &'static str, nym: &'static str) -> Self {
        Self {
            radio: None,
            circle_icon: Some(data),
            title: Some(name),
            title_flair: None,
            subtitle: Some(nym),
            description: None,
            title_right: None,
            subtitle_right: None,
        }
    }

    pub fn new_direct_message(data: CircleIconData, name: &'static str, recent: &'static str) -> Self {
        Self {
            radio: None,
            circle_icon: Some(data),
            title: Some(name),
            title_flair: None,
            subtitle: None,
            description: Some(recent),
            title_right: None,
            subtitle_right: None,
        }
    }

    pub fn new_group_message(names: Vec<&'static str>) -> Self {
        let description = Box::leak(names.join(", ").into_boxed_str());

        Self {
            radio: None,
            circle_icon: Some(CircleIconData::Icon(Icon::Group, IconStyle::Secondary)),
            title: Some("Group Message"),
            title_flair: None,
            subtitle: None,
            description: Some(description),
            title_right: None,
            subtitle_right: None,
        }
    } 

    pub fn new_room(data: CircleIconData, name: &'static str, members: &'static str, desc: &'static str) -> Self {
        Self {
            radio: None,
            circle_icon: Some(data),
            title: Some(name),
            title_flair: None,
            subtitle: Some(members),
            description: Some(desc),
            title_right: None,
            subtitle_right: None,
        }
    } 

    pub fn new_bitcoin(is_received: bool, usd: &'static str, date: &'static str) -> Self {
        let title = is_received { "Received Bitcoin" } else { "Sent Bitcoin" };
        Self {
            radio: None,
            circle_icon: None,
            title: Some(title),
            title_flair: None,
            subtitle: Some(date),
            description: None,
            title_right: Some(usd),
            subtitle_right: Some("Details"),
        }
    }

    pub fn new_bitcoin_sending(usd: &'static str, btc: &'static str, date: &'static str) -> Self {
        Self {
            radio: None,
            circle_icon: None,
            title: Some("Sending Bitcoin"),
            title_flair: Some((Icon::Warning, COLORS.status.warning)),
            subtitle: Some(date),
            description: None,
            title_right: Some(usd),
            subtitle_right: Some(btc),
        }
    }

    pub fn new_bitcoin_account(variant: AccountType, usd: &'static str, btc: &'static str) -> Self {
        let (icon, subtitle) = match variant {
            AccountType::Savings => (Icon::Safe, "Savings"),
            AccountType::Spending => (Icon::Wallet, "Spending")
        };

        Self {
            radio: None,
            circle_icon: Some(CircleIconData::Icon(icon, IconStyle::Brand)),
            title: Some("Wallet"),
            title_flair: None,
            subtitle: Some(subtitle),
            description: None,
            title_right: None,
            subtitle_right: None,
        }
    }

    pub fn new_selection(selected: bool, title: &'static str, subtitle: &'static str) -> Self {
        Self {
            radio: Some(selected),
            circle_icon: None,
            title: Some(title),
            title_flair: None,
            subtitle: Some(subtitle),
            description: None,
            title_right: None,
            subtitle_right: None,
        }
    }
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
