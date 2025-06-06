use pelican_ui::events::Event;
use pelican_ui::Context;

use crate::AppPage;
use crate::utils::ElementID;

/// Event used to navigate between pages of the app.
#[derive(Debug)]
pub struct NavigateEvent(pub Option<Box<dyn AppPage>>, pub bool);

impl NavigateEvent {
    pub fn new(page: (impl AppPage, bool)) -> Self {
        NavigateEvent(Some(page.0.into_boxed()), page.1)
    }
}

impl Event for NavigateEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, _children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        vec![if self.0.is_some() {Some(self)} else {None}]
    }
}

/// Event indicating whether the keyboard is visible or not.
#[derive(Debug, Clone)]
pub struct KeyboardActiveEvent(pub bool);

impl Event for KeyboardActiveEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

/// Event used to signal that a list item was selected.
#[derive(Debug, Clone)]
pub struct ListItemSelect(pub ElementID);

impl Event for ListItemSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

/// Event used to signal that a list item was selected.
#[derive(Debug, Clone)]
pub struct TextInputSelect(pub ElementID);

impl Event for TextInputSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

/// Event triggered when a navigation button is selected.
#[derive(Debug, Clone)]
pub struct NavigatorSelect(pub ElementID);

impl Event for NavigatorSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}


/// Event used to set the content of the currently active input field.
#[derive(Debug, Clone)]
pub struct SetActiveInput(pub String);

impl Event for SetActiveInput {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

