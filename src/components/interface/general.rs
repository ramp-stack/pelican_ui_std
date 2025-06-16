use pelican_ui::events::{OnEvent, MouseState, MouseEvent, Event};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::shapes::{Rectangle};
use crate::elements::text::TextStyle;
use crate::events::{TextInputSelect, NavigateEvent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size, Scroll};
use crate::components::avatar::{AvatarContent, AvatarRow};
use crate::components::button::{IconButton, Button};
use crate::components::text_input::TextInput;
use crate::elements::text::Text;
use crate::utils::{ElementID, AppPage};
use std::fmt::Debug;


use super::{DesktopInterface, MobileInterface};

pub type NavigateInfo = (&'static str, &'static str, Option<AvatarContent>, Box<dyn AppPage>);

#[derive(Debug, Component)]
pub struct Interface (Stack, Rectangle, Option<MobileInterface>, Option<DesktopInterface>);

impl Interface {
    pub fn new(
        ctx: &mut Context, 
        start_page: Box<dyn AppPage>,
        navigation: Option<(usize, Vec<NavigateInfo>)>,
    ) -> Self {
        let color = ctx.theme.colors.background.primary;
        let (mobile, desktop) = match crate::config::IS_MOBILE {
            true => (Some(MobileInterface::new(ctx, start_page, navigation)), None),
            false => (None, Some(DesktopInterface::new(ctx, start_page, navigation)))
        };
        Interface(Stack::default(), Rectangle::new(color), mobile, desktop)
    }
}

impl OnEvent for Interface {}

#[derive(Debug, Component)]
pub struct Page(Column, Header, Content, Option<Bumper>);
impl OnEvent for Page {}

impl Page {
    pub fn new(header: Header, content: Content, bumper: Option<Bumper>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[1].0, f32::MAX));
        Page(
            Column::new(12.0, Offset::Center, width, Padding::default()),
            header,
            content,
            bumper,
        )
    }

    pub fn header(&mut self) -> &mut Header {&mut self.1}
    pub fn content(&mut self) -> &mut Content {&mut self.2}
    pub fn bumper(&mut self) -> &mut Option<Bumper> {&mut self.3}
}

#[derive(Debug, Component)]
pub struct Content (Scroll, ContentChildren);

impl Content {
    pub fn new(offset: Offset, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0.min(375.0), 375.0));
        let height = Size::custom(move |_: Vec<(f32, f32)>|(0.0, f32::MAX));
        let mut layout = Scroll::new(Offset::Center, offset, width, height, Padding(24.0, 0.0, 24.0, 0.0));
        if offset == Offset::End { layout.set_scroll(f32::MAX); }
        Content(layout, ContentChildren::new(content)) 
    }
    
    pub fn find<T: std::any::Any>(&mut self) -> Option<&mut T> {
        self.items().iter_mut().find_map(|item| item.as_any_mut().downcast_mut::<T>())
    }

    pub fn find_at<T: std::any::Any>(&mut self, i: usize) -> Option<&mut T> {
        self.items().get_mut(i)?.as_any_mut().downcast_mut::<T>()
    }

    pub fn remove<T: std::any::Any>(&mut self) {
        if let Some(pos) = self.items().iter().position(|item| item.as_any().is::<T>()) {
            self.items().remove(pos);
        }
        // self.items().iter_mut().find_map(|item| item.as_any_mut().downcast_mut::<T>())
    }

    pub fn items(&mut self) -> &mut Vec<Box<dyn Drawable>> {&mut self.1.1}
    pub fn offset(&mut self) -> &mut Offset {self.0.offset()}
}

impl OnEvent for Content {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TextInputSelect(id)) = event.downcast_ref::<TextInputSelect>() {
            if crate::config::IS_MOBILE {
                let mut total_height = 0.0;
                for item in self.items().iter_mut() {
                    match item.as_any_mut().downcast_mut::<TextInput>() {
                        Some(input) if input.get_id() == *id => {
                            self.0.set_scroll(total_height);
                            break;
                        }
                        _ => {
                            let size = item.request_size(ctx);
                            total_height += size.max_height();
                        }
                    }
                }
            }
        } else if let Some(MouseEvent { state: MouseState::Scroll(_, y), position: Some(_) }) = event.downcast_ref::<MouseEvent>() {
            self.0.adjust_scroll(*y);
        }
        true
    }
}

#[derive(Debug, Component)]
struct ContentChildren (Column, Vec<Box<dyn Drawable>>);
impl OnEvent for ContentChildren {}

impl ContentChildren {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        ContentChildren(Column::center(24.0), content)
    }
}

#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl OnEvent for Header {}

impl Header {
    pub fn new(left: HeaderIcon, content: HeaderContent, right: HeaderIcon) -> Self {
        Header(
            Row::new(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            left, content, right,
        )
    }

    pub fn home(ctx: &mut Context, title: &str) -> Self {
        Header(
            Row::new(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(None), 
            HeaderContent::home(ctx, title),
            HeaderIcon::new(None)
        )
    }

    pub fn stack(
        ctx: &mut Context, 
        left: Option<IconButton>, 
        title: &str, 
        right: Option<IconButton>
    ) -> Self {
        Header(
            Row::new(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(left), 
            HeaderContent::stack(ctx, title), 
            HeaderIcon::new(right)
        )
    }
}

#[derive(Debug, Component)]
pub struct HeaderContent(Column, Option<AvatarRow>, Text);
impl OnEvent for HeaderContent {}

impl HeaderContent {
    pub fn new(avatar_row: Option<AvatarRow>, text: Text) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            avatar_row, text
        )
    }

    pub fn home(ctx: &mut Context, title: &str) -> Self {
        let text_size = ctx.theme.fonts.size.h3;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            None,
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
        )
    }

    pub fn stack(ctx: &mut Context, title: &str) -> Self {
        let text_size = ctx.theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()),  
            None,
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
        )
    }
}

#[derive(Debug, Component)]
pub struct HeaderIcon(Stack, Option<IconButton>);
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
pub struct Bumper (Stack, Rectangle, BumperContent);
impl OnEvent for Bumper {}

impl Bumper {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        let background = ctx.theme.colors.background.primary;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        let height = Size::custom(move |heights: Vec<(f32, f32)>|(heights[1].0, heights[1].1));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, height, Padding::default()),
            Rectangle::new(background), BumperContent::new(content)
        )
    }

    pub fn double_button(ctx: &mut Context, a: Button, b: Button) -> Self {
        Self::new(ctx, vec![Box::new(a), Box::new(b)])
    }

    pub fn single_button(ctx: &mut Context, a: Button) -> Self {
        Self::new(ctx, vec![Box::new(a)])
    }

    pub fn input(ctx: &mut Context, input: TextInput) -> Self {
        Self::new(ctx, vec![Box::new(input)])
    }

    pub fn items(&mut self) -> &mut Vec<Box<dyn Drawable>> {
        &mut self.2.1
    }

    pub fn find<T: std::any::Any>(&mut self) -> Option<&mut T> {
        self.items().iter_mut().find_map(|item| item.as_any_mut().downcast_mut::<T>())
    }

    pub fn find_at<T: std::any::Any>(&mut self, i: usize) -> Option<&mut T> {
        self.items().get_mut(i)?.as_any_mut().downcast_mut::<T>()
    }
}

#[derive(Debug, Component)]
struct BumperContent (Row, Vec<Box<dyn Drawable>>);

impl BumperContent {
    fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row::new(16.0, Offset::Center, Size::Fit, Padding(0.0, 24.0, 0.0, 24.0)), content)
    }
}

impl OnEvent for BumperContent {}

#[derive(Debug, Component)]
pub struct NavigationButton(Stack, Option<Button>, Option<IconButton>, #[skip] ElementID);

impl OnEvent for NavigationButton {}

impl NavigationButton {
    pub fn new(id: ElementID, button: Option<Button>, icon_button: Option<IconButton>) -> Self {
        NavigationButton(Stack::default(), button, icon_button, id)
    }

    pub fn id(&self) -> ElementID {
        self.3
    }

    pub fn button(&mut self) -> &mut Option<Button> {
        &mut self.1
    }

    pub fn icon_button(&mut self) -> &mut Option<IconButton> {
        &mut self.2
    }
}
