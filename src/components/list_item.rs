use rust_on_rails::prelude::*;
use crate::theme;
use crate::{Row, Column, Stack, Text};
use crate::layout::Align;
use crate::components::abc::CircleIcon;

pub struct ListItem {
    icon: Option<&'static str>,
    title_flair: Option<&'static str>,
    description: Option<&'static str>,
    subtitle: Option<&'static str>,
    subtitle_right: Option<&'static str>,
    title: Option<&'static str>,
    title_right: Option<&'static str>,
    circle_icon: Option<CircleIcon>,
}

impl ListItem {
    pub fn conversation(name: &'static str, recent: &'static str) -> Self {
        Self {
            icon: None,
            title_flair: None,
            description: Some(recent),
            subtitle: None,
            subtitle_right: None,
            title: Some(name),
            title_right: None,
            circle_icon: Some(CircleIcon::Photo("profile", 48)), // get user pfp
        }
    }

    pub fn transaction(usd_amount: &'static str, date_time: &'static str, is_received: bool, is_pending: bool) -> Self {
        let title = if is_received {"Received Bitcoin"} else {"Send Bitcoin"};
        Self {
            icon: None,
            title_flair: None,
            description: None,
            subtitle: Some(date_time),
            subtitle_right: None,
            title: Some(title),
            title_right: Some(usd_amount),
            circle_icon: None,
        }
    }
    
    pub fn user(name: &'static str, nym: &'static str) -> Self {
        Self {
            icon: None,
            title_flair: None,
            description: None,
            subtitle: Some(nym),
            subtitle_right: None,
            title: Some(name),
            title_right: None,
            circle_icon: Some(CircleIcon::Photo("profile", 48)), // get user pfp
        }
    }
}

pub enum ListItemType {
    Transaction,
    PendingTransaction,
    Wallet,
    Conversation,
    Room,
    User,
}


impl ComponentBuilder for ListItem {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap(); // Get Heading
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap(); // Get Text
        let colors = theme::color::palette();
        let title_flair = ctx.load_image("images/profile.png").unwrap(); // Get self.title_flair image
        let icon = ctx.load_image("images/profile.png").unwrap(); // Get self.icon image
        
        let mut left_column: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut right_column: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut row_items: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut title_row: Vec<Box< dyn ComponentBuilder>> = vec![];

        if let Some(title) = &self.title {
            title_row.push(Box::new(Text::new(title.clone(), colors.text.heading, 16, heading.clone())));
            if let Some(_t_flair) = &self.title_flair {
                title_row.push(Box::new(Image(ShapeType::Rectangle(20, 20), title_flair)));
            }
            left_column.push(Box::new(Row!(8, Vec2::new(0, 0), Align::Center, true, title_row)));
        }

        if let Some(subtitle) = &self.subtitle {
            left_column.push(Box::new(Text::new(subtitle.clone(), colors.text.primary, 14, font.clone())));
        }

        if let Some(description) = &self.description {
            left_column.push(Box::new(Text::new(description.clone(), colors.text.secondary, 12, font.clone())));
        }

        if let Some(title_right) = &self.title_right {
            right_column.push(Box::new(Text::new(title_right.clone(), colors.text.heading, 16, heading.clone())));
        }

        if let Some(subtitle_right) = &self.subtitle_right {
            right_column.push(Box::new(Text::new(subtitle_right.clone(), colors.text.primary, 14, font.clone())));
        }

        if let Some(circle_icon) = self.circle_icon {
            row_items.push(Box::new(circle_icon));
        }

        row_items.push(Box::new(
            Row!(16, Vec2::new(0, 0), Align::Top, true, vec![
                Box::new(Column!(4, Vec2::new(0, 0), Align::Left, true, left_column)),
                Box::new(Column!(4, Vec2::new(0, 0), Align::Right, true, right_column))
            ])
        ));

        if let Some(i) = self.icon {
            row_items.push(Box::new(Image(ShapeType::Rectangle(24, 24), icon)));
        }

        Row!(16, Vec2::new(0, 0), Align::Center, true, row_items).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}