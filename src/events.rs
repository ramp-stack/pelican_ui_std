use rust_on_rails::prelude::*;

// #[derive(Debug, Clone, Copy)]
// pub struct NavigationEvent;
// impl Event for NavigationEvent {
//     fn pass(self: Box<Self>, _ctx: &mut ComponentContext, children: Vec<((i32, i32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
//         children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct SummonKeyboardEvent;
impl Event for SummonKeyboardEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((i32, i32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HideKeyboardEvent;
impl Event for HideKeyboardEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((i32, i32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListItemSelect(pub uuid::Uuid);
impl Event for ListItemSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((i32, i32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}