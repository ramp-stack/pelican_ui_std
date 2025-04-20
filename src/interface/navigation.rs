use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::events::NavigatorSelect;
use crate::elements::images::Brand;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::{Button, IconButton, ButtonColumn, ButtonState};
use crate::components::avatar::{AvatarIconStyle, AvatarContent, AvatarRow};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::{PelicanUI, ElementID};

#[derive(Debug, Component)]
pub struct MobileNavigator(Row, Vec<IconButton>);

impl MobileNavigator {
    pub fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>), 
        mut profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>)
    ) -> Self {
        if navigation.1.is_empty() {panic!("MobileNavigator: Parameter 1 was empty. Navigator has no data.")}
        let profile_id = ElementID::new();

        let mut tabs: Vec<IconButton> = navigation.1.into_iter().enumerate().map(|(y, (i, _, mut c))| {
            let id = ElementID::new();
            IconButton::tab_nav(ctx, i, y == navigation.0, id, move |ctx: &mut Context| {
                println!("triggered");
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            })
        }).collect();

        tabs.push(
            IconButton::tab_nav(ctx, "profile", false, profile_id, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(profile_id));
                (profile.2)(ctx);
            }),
        );

        MobileNavigator(Row(48.0, Offset::Center, Size::Fit, Padding(0.0, 8.0, 0.0, 8.0)), tabs)
    }
}

impl Events for MobileNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            self.1.iter_mut().for_each(|button| {
                if button.id().unwrap() == *id {
                    *button.status() = ButtonState::Selected;
                } else {
                    *button.status() = ButtonState::UnSelected;
                    button.color(ctx, ButtonState::UnSelected);
                }
            });
        }
        true
    }
}



#[derive(Debug, Component)]
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, Button);

impl DesktopNavigator {
    pub fn new(
        ctx: &mut Context, 
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>), 
        mut profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>)
    ) -> Self {
        if navigation.1.is_empty() {panic!("DesktopNavigator: Parameter 1 was empty. Navigator has no data.")}

        let theme = &ctx.get::<PelicanUI>().theme;
        let (wordmark, color) = (theme.brand.wordmark.clone(), theme.colors.shades.transparent);
        let profile_id = ElementID::new();

        let mut tabs: Vec<Button> = navigation.1.into_iter().enumerate().map(|(y, (i, n, mut c))| {
            let id = ElementID::new();
            Button::navigation(ctx, i, n, y == navigation.0, id, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            })
        }).collect();

        DesktopNavigator(
            Column(32.0, Offset::Center, Size::Fill(100.0, 200.0), Padding(16.0, 32.0, 16.0, 32.0)),
            Brand::new(wordmark, (80.0, 44.0)),
            ButtonColumn::new(tabs),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100.0, 200.0), Size::Fill(100.0, f32::MAX), Padding::default()), 
                Rectangle::new(color)
            ),
            Button::photo(ctx, profile.0, profile.1, false, profile_id, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(profile_id));
                (profile.2)(ctx);
            }),
        )
    }
}

impl Events for DesktopNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            let mut buttons: Vec<&mut Button> = self.2.buttons().iter_mut().map(|btn| btn).collect();
            buttons.push(&mut self.4);
            buttons.iter_mut().for_each(|button| {
                if button.id().unwrap() == *id {
                    *button.status() = ButtonState::Selected;
                    button.color(ctx);
                } else {
                    *button.status() = ButtonState::Default;
                    button.color(ctx);
                }
            });
        }
        true
    }
}


#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl Events for Header {}

impl Header {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(None), 
            HeaderContent::home(ctx, title),
            HeaderIcon::new(None)
        )
    }

    pub fn stack(
        ctx: &mut Context, 
        left: Option<IconButton>, 
        title: &'static str, 
        right: Option<IconButton>
    ) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(left), 
            HeaderContent::stack(ctx, title), 
            HeaderIcon::new(right)
        )
    }

    pub fn chat(
        ctx: &mut Context, 
        left: Option<IconButton>,
        right: Option<IconButton>,
        avatars: Vec<AvatarContent>,
    ) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(left), 
            HeaderContent::chat(ctx, avatars), 
            HeaderIcon::new(right)
        )
    }
}

#[derive(Debug, Component)]
struct HeaderContent(Column, BasicText, Option<AvatarRow>);
impl Events for HeaderContent {}

impl HeaderContent {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h3;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, TextAlign::Left),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()),  
            Text::new(ctx, title, TextStyle::Heading, text_size, TextAlign::Left),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, TextAlign::Left),
            Some(AvatarRow::new(ctx, avatars)),
        )
    }
}

#[derive(Debug, Component)]
struct HeaderIcon(Stack, Option<IconButton>);
impl Events for HeaderIcon {}

impl HeaderIcon {
    pub fn new(icon: Option<IconButton>) -> Self {
        HeaderIcon(
            Stack(Offset::Center, Offset::Center, Size::Static(48.0), Size::Static(48.0), Padding::default()),
            icon
        )
    }
}


#[derive(Debug, Component)]
pub struct Bumper (Stack, BumperContent);
impl Events for Bumper {}

impl Bumper {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            BumperContent::new(content)
        )
    }
}

#[derive(Debug, Component)]
pub struct BumperContent (Row, Vec<Box<dyn Drawable>>);
impl Events for BumperContent {}

impl BumperContent {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row::center(16.0), content)
    }
}