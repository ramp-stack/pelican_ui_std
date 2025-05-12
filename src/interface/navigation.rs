use rust_on_rails::prelude::*;
use crate::events::NavigatorSelect;
use crate::elements::images::Brand;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::{Button, IconButton, ButtonState};
use crate::components::avatar::{AvatarContent, AvatarRow};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::{PelicanUI, ElementID};

#[derive(Debug, Component)]
pub struct MobileNavigator(Row, Vec<NavigationButton>);

impl MobileNavigator {
    pub fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>), 
        mut profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>)
    ) -> Self {
        if navigation.1.is_empty() {panic!("MobileNavigator: Parameter 1 was empty. Navigator has no data.")}
        let profile_id = ElementID::new();

        let mut tabs: Vec<NavigationButton> = navigation.1.into_iter().enumerate().map(|(y, (i, _, mut c))| {
            let id = ElementID::new();
            let ib = IconButton::tab_nav(ctx, i, y == navigation.0, move |ctx: &mut Context| {
                println!("triggered");
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            });
            NavigationButton::new(id, None, Some(ib))
        }).collect();

        let ib = IconButton::tab_nav(ctx, "profile", false, move |ctx: &mut Context| {
            ctx.trigger_event(NavigatorSelect(profile_id));
            (profile.2)(ctx);
        });

        tabs.push(NavigationButton::new(profile_id, None, Some(ib)));

        MobileNavigator(Row(48.0, Offset::Center, Size::Fit, Padding(0.0, 8.0, 0.0, 8.0)), tabs)
    }
}

impl OnEvent for MobileNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            self.1.iter_mut().for_each(|button| {
                *button.1.as_mut().unwrap().status() = if button.id() == *id {ButtonState::Selected} else {ButtonState::UnSelected};
                button.1.as_mut().unwrap().color(ctx);
            });
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, NavigationButton);

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

        let tabs: Vec<NavigationButton> = navigation.1.into_iter().enumerate().map(|(y, (i, n, mut c))| {
            let id = ElementID::new();
            let nb = Button::navigation(ctx, i, n, y == navigation.0, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            });
            NavigationButton::new(id, Some(nb), None)
        }).collect();

        let pb = Button::photo(ctx, profile.0, profile.1, false, move |ctx: &mut Context| {
            ctx.trigger_event(NavigatorSelect(profile_id));
            (profile.2)(ctx);
        });

        DesktopNavigator(
            Column::new(32.0, Offset::Center, Size::Fill(100.0, 200.0), Padding(16.0, 32.0, 16.0, 32.0)),
            Brand::new(wordmark, (80.0, 44.0)),
            ButtonColumn::new(tabs),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100.0, 200.0), Size::Fill(100.0, f32::MAX), Padding::default()), 
                Rectangle::new(color)
            ),
            NavigationButton::new(profile_id, Some(pb), None)
        )
    }
}

impl OnEvent for DesktopNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            let mut buttons: Vec<&mut NavigationButton> = self.2.buttons().iter_mut().map(|btn| btn).collect();
            buttons.push(&mut self.4);
            buttons.iter_mut().for_each(|button| {
                *button.1.as_mut().unwrap().status() = if button.id() == *id {ButtonState::Selected} else {ButtonState::Default};
                button.1.as_mut().unwrap().color(ctx);
            });
        }
        true
    }
}

#[derive(Debug, Component)]
struct ButtonColumn(Column, Vec<NavigationButton>);
impl OnEvent for ButtonColumn {}

impl ButtonColumn {
    fn new(buttons: Vec<NavigationButton>) -> Self {
        ButtonColumn(Column::center(8.0), buttons)
    }

    fn buttons(&mut self) -> &mut Vec<NavigationButton> {&mut self.1}
}

#[derive(Debug, Component)]
struct NavigationButton(Stack, Option<Button>, Option<IconButton>, #[skip] ElementID);
impl OnEvent for NavigationButton {}

impl NavigationButton {
    pub fn new(id: ElementID, button: Option<Button>, icon_button: Option<IconButton>) -> Self {
        NavigationButton(Stack::default(), button, icon_button, id)
    }

    pub fn id(&self) -> ElementID {self.3}
}

#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl OnEvent for Header {}

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
struct HeaderContent(Column, Text, Option<AvatarRow>);
impl OnEvent for HeaderContent {}

impl HeaderContent {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h3;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()),  
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            Some(AvatarRow::new(ctx, avatars)),
        )
    }
}

#[derive(Debug, Component)]
struct HeaderIcon(Stack, Option<IconButton>);
impl OnEvent for HeaderIcon {}

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
impl OnEvent for Bumper {}

impl Bumper {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            BumperContent::new(content)
        )
    }

    pub fn double_button(a: Button, b: Button) -> Self {
        Self::new(vec![Box::new(a), Box::new(b)])
    }

    pub fn single_button(a: Button) -> Self {
        Self::new(vec![Box::new(a)])
    }

    // pub fn message_input(a: TextInput) -> Self {
    //     Self::new(vec![Box::new(a)])
    // }
    pub fn items(&mut self) -> &mut Vec<Box<dyn Drawable>> {&mut self.1.1}
}

#[derive(Debug, Component)]
struct BumperContent (Row, Vec<Box<dyn Drawable>>);
impl OnEvent for BumperContent {}

impl BumperContent {
    fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row::center(16.0), content)
    }
}