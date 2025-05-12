use rust_on_rails::prelude::*;
use crate::{AppFlow, ElementID};

/// Event used to navigate between pages of the app.
#[derive(Debug, Clone)]
pub struct NavigateEvent(pub Box<dyn AppFlow>);

impl Event for NavigateEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self.clone()) as Box<dyn Event>)).collect()
    }
}

/// Event indicating whether the keyboard is visible or not.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardActiveEvent(pub bool);

impl Event for KeyboardActiveEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

/// Event used to signal that a list item was selected.
#[derive(Debug, Clone, Copy)]
pub struct ListItemSelect(pub ElementID);

impl Event for ListItemSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

/// Event triggered when a navigation button is selected.
#[derive(Debug, Clone, Copy)]
pub struct NavigatorSelect(pub ElementID);

impl Event for NavigatorSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

/// Event to add a contact to a `QuickDeselect` component.
#[derive(Debug, Clone, Copy)]
pub struct AddContactEvent(pub &'static str, pub ElementID);

impl Event for AddContactEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

/// Event to remove a contact from a `QuickDeselect` component.
#[derive(Debug, Clone, Copy)]
pub struct RemoveContactEvent(pub ElementID);

impl Event for RemoveContactEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

/// Event used to set the content of the currently active input field.
#[derive(Debug, Clone, Copy)]
pub struct SetActiveInput(pub &'static str);

impl Event for SetActiveInput {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}
