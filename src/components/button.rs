use rust_on_rails::prelude::*;
use crate::theme;
use crate::{Row, Text, Column};

// #[derive(Clone)]
// pub struct Button {
//     pub shape: Shape,
//     pub style: ButtonStyle, 
//     pub size: Size, 
//     pub width: Width, 
//     pub icon: Option<&'static str>, 
//     pub label: &'static str
// }

// impl ComponentBuilder for Button {
//     fn build_children(&mut self, ctx: &mut Context, parent_size: Vec2)  {
//         let colors = palette().button.colors_from(self.style, ButtonState::Default);

//         let (text_size, height, icon_size) = match self.size {
//             Size::Medium => (32.0, px(ctx, 32.0), px(ctx, 16.0)),
//             Size::Large => (48.0, px(ctx, 48.0), px(ctx, 24.0))
//         };

//         let mut label = CustomText::label(self.label, text_size)
//             .build(ctx, Rect::new(0.0, 0.0, parent_size.x, parent_size.y), true)?;

//         let label_size = label.size(ctx);

//         let width = match self.width {
//             Width::Hug => label_size.x + 48.0,
//             Width::Expand => parent_size.x
//         };

//         label.1.x = (width - label_size.x) / 2.0;
//         label.1.y = (height - label_size.y) / 2.0;
        
//         Ok(vec![
//             Box::new(
//                 Rectangle {
//                     height,
//                     width,
//                     radius: 50.0,
//                     stroke: colors.outline,
//                     color: colors.background,
//                 }.build(ctx, Rect::new(0.0, 0.0, parent_size.x, parent_size.y), true)?,
//             ),
//             Box::new(label)
//         ])
//     }
// }

#[derive(Clone)]
pub struct Button {
    // pub icon: Option<&'static str>, 
    pub label: &'static str,
    pub style: ButtonStyle,
    pub size: Size,
    pub width: Width
}

impl Button {
    pub fn primary(label: &'static str, size: Size, width: Width) -> Self {
        Self { label, style: ButtonStyle::Primary, size, width }
    }

    pub fn secondary(label: &'static str, size: Size, width: Width) -> Self {
        Self { label, style: ButtonStyle::Secondary, size, width }
    }

    pub fn ghost(label: &'static str, size: Size, width: Width) -> Self {
        Self { label, style: ButtonStyle::Ghost, size, width }
    }
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let colors = theme::color::palette().button.colors_from(self.style, ButtonState::Default);
        let font = ctx.load_font("fonts/outfit_bold.ttf").unwrap(); // GET LABEL FONT
        let image = ctx.load_image("icons/pfp.png").unwrap(); // GET DESIRED ICON

        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (text_size, height, icon_size) = match self.size {
            Size::Medium => (16, 32, 16),
            Size::Large => (20, 48, 24)
        };

        let mut content = Row!(3, 
            // Image(ShapeType::Rectangle(icon_size, 8), image),
            Text::new(self.label, colors.label, text_size, font.clone())
        ).build(ctx, bound);

        let width = match self.width {
            Width::Hug => content.size(ctx).x + 48,
            Width::Expand => max_size.x,
        };

        content.1 = Rect::new((width - content.size(ctx).x) / 2, (height - content.size(ctx).y) / 2, max_size.x, max_size.y);

        vec![
            Box::new(Shape(ShapeType::Rectangle(width, height), colors.background, None).build(ctx, bound)),
            Box::new(Shape(ShapeType::Rectangle(width, height), colors.outline, Some(200)).build(ctx, bound)),
            Box::new(content)
        ]
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone)]
pub enum Width {
    Expand,
    Hug,
}

#[derive(Debug, Clone)]
pub enum Size {
    Large,
    Medium,
}

// #[derive(PartialEq, Eq, Clone)]
// pub enum Alignment {
//     Left,
//     Right,
//     Top,
//     Bottom,
//     Center,
// }
