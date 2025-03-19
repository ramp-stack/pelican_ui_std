use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Column, COLORS, Align, ConstrainedBox, Row, ZERO, Expand };
use crate::components::ProfilePictures;

#[derive(Clone, Copy)]
pub enum Header {
    Home(&'static str),
    Stack(Option<Icon>, &'static str, Option<Icon>),
    Chat(Vec<Profile>) 
}

impl ComponentBuilder for Header {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let icon_button = |icon: Icon| {
            IconButton(icon, IconButtonStyle::Ghost, Size::Medium);
        };

        match self {
            Header::Home(title) => {
                Row(ZERO, AUTO, Align::Center, vec![
                    Spacer(32), // Icon Spacer
                    Text::heading(ctx, title, TextSize::h4()), // Page Title
                    Spacer(32) // Icon Spacer
                ])
            },
            Header::Stack(l, title, r) = {
                Row(ZERO, AUTO, Align::Center, vec![
                    if let Some(icon) = l { icon_button(icon) } else { Spacer(32) }, // Icon or Spacer
                    Text::heading(ctx, title, TextSize::h4()), // Page Title
                    if let Some(icon) = r { icon_button(icon) } else { Spacer(32) } // Icon or Spacer
                ])
            },
            Header::Chat(members) => {
                Row(ZERO, AUTO, Align::Center, vec![
                    icon_button(Icon::Back), // Back Button
                    Column(ZERO, 10, Align::Center, vec![
                        match members.len() == 1 { 
                            true => Text::heading(ctx, members.0.name, TextSize::h4()), // DM (username)
                            false => Text::heading(ctx, "Group Message", TextSize::h4()), // Group Message
                        },
                        Stack(ZERO, Align::Left, members
                            .iter().take(5)
                            .enumerate()
                            .map(|profile| {
                                CircleIcon(profile.profile_photo, None, true, 32) // Profile Photo Stack
                            }).collect()
                        )
                    ]),
                    icon_button(Icon::Info) // Info Button
                ])
            }
        }.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}