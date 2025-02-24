use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Row, Column, ZERO, Align };
use crate::components::{UserIcon, CircleIcon};

impl ListItem {
    pub fn user(name: &'static str, nym: &'static str) -> Self {
        Self {
            // radio: None,
            circle_icon: None, // Some(CircleIcon::Brand("profile", 48)),
            user_icon: Some(UserIcon("profile", 48, None)),
            title: Some(name),
            subtitle: Some(nym),
            title_flair: None,
            description: None,
            title_right: None,
            subtitle_right: None
        }
    }
}

pub struct ListItem {
    // radio: Option<RadioButton>,
    user_icon: Option<UserIcon>,
    circle_icon: Option<CircleIcon>,
    title: Option<&'static str>,
    title_flair: Option<&'static str>,
    subtitle: Option<&'static str>,
    description: Option<&'static str>,
    title_right: Option<&'static str>,
    subtitle_right: Option<&'static str>,
}

impl ComponentBuilder for ListItem {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources"));
        let title_flair = ctx.load_image("images/profile.png").unwrap(); // Get self.title_flair image
        
        let mut list_item: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut title_row: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut details: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut details_left: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut details_right: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];

        // if let Some(radio) = &self.radio { list_item.push((Child!(*radio), false)); }
        if let Some(circle_icon) = &self.circle_icon { list_item.push((Child!(*circle_icon), false)); }
        if let Some(user_icon) = &self.user_icon { list_item.push((Child!(*user_icon), false)); }
        
        if let Some(title) = &self.title { title_row.push((Child!(Text::heading(ctx, title, TextSize::h5())), false)); }
        if let Some(_flair) = &self.title_flair { title_row.push((Child!(Image(ShapeType::Rectangle(20, 20), title_flair)), false)); }

        details_left.push((Child!(Row { children: title_row, align: Align::Left, spacing: 8, padding: ZERO }), false));

        if let Some(subtitle) = &self.subtitle { details_left.push((Child!(Text::secondary(ctx, subtitle, TextSize::xs())), false)); }
        if let Some(description) = &self.description { details_left.push((Child!(Text::secondary(ctx, description, TextSize::xs())), false)); }

        if let Some(title) = &self.title_right { details_right.push((Child!(Text::heading(ctx, title, TextSize::h5())), false)); }
        if let Some(subtitle) = &self.subtitle_right { details_right.push((Child!(Text::secondary(ctx, subtitle, TextSize::xs())), false)); }
        
        details.push((Child!(Column { children: details_left, align: Align::Left, spacing: 4, padding: ZERO }), true));
        details.push((Child!(Column { children: details_right, align: Align::Right, spacing: 4,  padding: ZERO }), false));

        list_item.push((Child!(Row { children: details, align: Align::Left, spacing: 8, padding: ZERO }), false));

        Row { children: list_item, align: Align::Center, spacing: 16, padding: ZERO }.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}