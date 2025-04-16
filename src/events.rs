use rust_on_rails::prelude::*;
use crate::PageName;

#[derive(Debug, Clone)]
pub struct NavigateEvent(pub Box<dyn PageName>, pub bool);
impl Event for NavigateEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self.clone()) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SummonKeyboardEvent;
impl Event for SummonKeyboardEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HideKeyboardEvent;
impl Event for HideKeyboardEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListItemSelect(pub uuid::Uuid);
impl Event for ListItemSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NavigatorSelect(pub uuid::Uuid);
impl Event for NavigatorSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddContactEvent(pub &'static str, pub uuid::Uuid);
impl Event for AddContactEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RemoveContactEvent(pub uuid::Uuid);
impl Event for RemoveContactEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}