use rust_on_rails::prelude::*;
use crate::components::inputs::*;
use crate::{
    ConstrainedBox,
    Row, 
    Column, 
    Stack, 
    Text, 
    Padding,
    COLORS
};
use crate::layout::Align;

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

    pub fn transaction(usd_amount: &'static str, date_time: &'static str, is_received: bool, _is_pending: bool) -> Self {
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
        let title_flair = ctx.load_image("images/profile.png").unwrap(); // Get self.title_flair image
        let icon = ctx.load_image("images/profile.png").unwrap(); // Get self.icon image
        
        let mut left_column: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut right_column: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut row_items: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut title_row: Vec<Box< dyn ComponentBuilder>> = vec![];

        if let Some(title) = &self.title {
            title_row.push(Box::new(Text::new(title, COLORS.text.heading, 16, heading.clone())));
            if let Some(_t_flair) = &self.title_flair {
                title_row.push(Box::new(Image(ShapeType::Rectangle(20, 20), title_flair)));
            }
            left_column.push(Box::new(Row!(8, Vec2::new(0, 0), Align::Center, true, title_row)));
        }

        if let Some(subtitle) = &self.subtitle {
            left_column.push(Box::new(Text::new(subtitle, COLORS.text.primary, 14, font.clone())));
        }

        if let Some(description) = &self.description {
            left_column.push(Box::new(Text::new(description, COLORS.text.secondary, 12, font.clone())));
        }

        if let Some(title_right) = &self.title_right {
            right_column.push(Box::new(Text::new(title_right, COLORS.text.heading, 16, heading.clone())));
        }

        if let Some(subtitle_right) = &self.subtitle_right {
            right_column.push(Box::new(Text::new(subtitle_right, COLORS.text.primary, 14, font.clone())));
        }

        if let Some(circle_icon) = self.circle_icon {
            row_items.push(Box::new(circle_icon));
        }

        row_items.push(Box::new(
            Row!(16, Vec2::new(0, 0), Align::Top, false,
                Column!(4, Vec2::new(0, 0), Align::Left, true, left_column),
                Column!(4, Vec2::new(0, 0), Align::Right, true, right_column)
            )
        ));

        if let Some(_i) = self.icon {
            row_items.push(Box::new(Image(ShapeType::Rectangle(24, 24), icon)));
        }

        Row!(16, Vec2::new(0, 0), Align::Center, true, row_items).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Clone, Copy)]
pub enum CircleIcon {
    Icon(&'static str, u32),
    Photo(&'static str, u32),
    Brand(&'static str, u32)
}

impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
        let image = ctx.load_image("images/profile.png").unwrap(); // Get individual path images

        match self {
            CircleIcon::Photo(_, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.background.primary, Some(100)), Vec2::new(0, 0)),
                    (Image(ShapeType::Circle(*s / 2), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Icon(_, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.background.secondary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Brand(_, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.brand.primary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            }
        }
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Card {
    circle_icon: CircleIcon,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    button: Button
}

impl Card {
    pub fn room(n: &'static str, st: &'static str, d: &'static str) -> Self {
        Self {
            circle_icon: CircleIcon::Photo("profile", 64), // get user pfp
            title: n,
            subtitle: st,
            description: d,
            button: Button::Secondary("Join Room", Size::Medium, Width::Hug, None, Align::Center),
        }
    }
}

impl ComponentBuilder for Card {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        Column!(8, Vec2::new(24, 16), Align::Center, false,
            self.circle_icon,
            Text::new(self.title, COLORS.text.heading, 24, heading.clone()),
            Text::new(self.subtitle, COLORS.text.primary, 12, font.clone()),
            Padding(Vec2::new(1, 6), COLORS.background.primary),
            Shape(ShapeType::Rectangle(230, 1), COLORS.outline.secondary, None),
            Padding(Vec2::new(1, 6), COLORS.background.primary),
            Text::new(self.description, COLORS.text.primary, 14, font.clone()),
            self.button
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


pub struct ProfilePictures(pub Vec<&'static str>);

impl ComponentBuilder for ProfilePictures {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); 
        let image = ctx.load_image("images/profile.png").unwrap();

        let pfps: Vec<(Box<dyn ComponentBuilder>, Vec2)> = self.0
            .iter()
            .take(5)
            .enumerate()
            .map(|(index, _)| (
                Box::new(Stack!(Vec2::new(0, 0), Align::Center, 
                    (Image(ShapeType::Circle(32 / 2), image.clone()), Vec2::new(0, 0)),
                    (Shape(ShapeType::Circle(32 / 2), COLORS.background.primary, Some(500)), Vec2::new(0, 0))
                )) as Box<dyn ComponentBuilder>,
                Vec2::new(index as u32 * 20, 0)
            ))
            .collect();

        Stack!(Vec2::new(0, 0), Align::Left, true, pfps).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct AmountDisplay(pub &'static str, pub Option<&'static str>);

impl ComponentBuilder for AmountDisplay {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let text = ctx.load_font("fonts/outfit_regular.ttf").unwrap();

        let mut column: Vec<Box<dyn ComponentBuilder>> = vec![];

        column.push(Box::new(Text::new(self.0, COLORS.text.heading, 48, heading.clone())));

        if let Some(err) = &self.1 {
            column.push(Box::new(Row!(8, Vec2::new(0, 0), Align::Left, false,
                (Shape(ShapeType::Rectangle(32, 32), COLORS.status.danger, None), false),
                (Text::new(err, COLORS.status.danger, 20, text.clone()), false)
            )));
        } else {
            column.push(Box::new(Text::new("0.00001234 BTC", COLORS.text.secondary, 20, text.clone())));
        }

        ConstrainedBox!(300, 
            Column!(32, Vec2::new(16, 32), Align::Center, true, column)
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Clone)]
pub enum MessageBubble {
    You(&'static str),
    ContactGroup(&'static str),
    Rooms(&'static str)
}

impl ComponentBuilder for MessageBubble {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap(); // GET TEXT FONT

        let (background, text_color, text, pad) = match self {
            MessageBubble::You(a) 
                => (COLORS.brand.primary, COLORS.text.heading, *a, 24),
            MessageBubble::ContactGroup(a) 
                => (COLORS.background.secondary, COLORS.text.primary, *a, 24),
            MessageBubble::Rooms(a) 
                => (COLORS.background.primary, COLORS.text.primary, *a, 0)
        };

        let bound = Rect::new(0, 0, max_size.x, max_size.y);
        let mut content = Text::new(text, text_color, 16, font.clone()).build(ctx, bound);

        let (width, height) = (content.size(ctx).x + pad, content.size(ctx).y + pad);

        content.1 = Rect::new((width - content.size(ctx).x) / 2, (width - content.size(ctx).x) / 2, max_size.x, max_size.y);

        vec![
            Box::new(Shape(ShapeType::Rectangle(width, height), background, None).build(ctx, bound)),
            Box::new(content)
        ]
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct TextMessage(pub MessageType, pub Vec<&'static str>);

pub enum MessageType {
    You,
    Contact,
    Group,
    Rooms,
}

impl ComponentBuilder for TextMessage {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();

        let boxed_messages: Vec<Box<dyn ComponentBuilder>> = self.1.iter()
            .map(|msg| {
                let bubble = match self.0 {
                    MessageType::You => MessageBubble::You(msg),
                    MessageType::Contact | MessageType::Group => MessageBubble::ContactGroup(msg),
                    MessageType::Rooms => MessageBubble::Rooms(msg),
                };

                Box::new(bubble) as Box<dyn ComponentBuilder>
            }).collect();
    
        match self.0 {
            MessageType::You => {
                Column!(8, Vec2::new(0, 0), Align::Right, false,
                    Column!(8, Vec2::new(0, 0), Align::Left, true, boxed_messages),
                    Text::new("11:48 AM", COLORS.text.secondary, 14, font.clone())
                ).build_children(ctx, max_size)
            },
            MessageType::Contact => {
                Column!(8, Vec2::new(0, 0), Align::Left, false,
                    Column!(8, Vec2::new(0, 0), Align::Left, true, boxed_messages),
                    Text::new("11:48 AM", COLORS.text.secondary, 14, font.clone())
                ).build_children(ctx, max_size)
            },
            MessageType::Group => {
                Row!(8, Vec2::new(0, 0), Align::Bottom, false,
                    CircleIcon::Photo("profile.png", 24),
                    Column!(8, Vec2::new(0, 0), Align::Left, false, 
                        Column!(8, Vec2::new(0, 0), Align::Left, true, boxed_messages),
                        Row!(4, Vec2::new(0, 0), Align::Bottom, false,
                            Text::new("Ella Couch", COLORS.text.secondary, 14, font.clone()),
                            Text::new("·", COLORS.text.secondary, 14, font.clone()),
                            Text::new("11:48 AM", COLORS.text.secondary, 14, font.clone())
                        )
                    )
                ).build_children(ctx, max_size)
            },
            MessageType::Rooms => {
                Row!(8, Vec2::new(0, 0), Align::Top, false,
                    CircleIcon::Photo("profile.png", 24),
                    Column!(8, Vec2::new(0, 0), Align::Left, false,
                        Row!(4, Vec2::new(0, 0), Align::Bottom, false,
                            Text::new("Ella Couch", COLORS.text.heading, 16, heading.clone()),
                            Text::new("·", COLORS.text.secondary, 14, font.clone()),
                            Text::new("11:48 AM", COLORS.text.secondary, 14, font.clone())
                        ),
                        Column!(8, Vec2::new(0, 0), Align::Left, true, boxed_messages)
                    )
                ).build_children(ctx, max_size)
            }
        }

    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}