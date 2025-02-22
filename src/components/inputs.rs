use rust_on_rails::prelude::*;
use crate::{
    ConstrainedBox,
    Row, 
    Column, 
    Stack, 
    Text, 
    Expand,
    COLORS
};
use crate::layout::Align;

// == Button -- //

#[derive(Clone, Copy)]
pub struct Button(&'static str, Size, Width, Option<&'static str>, ButtonStyle, Align);

impl Button {
    pub fn Primary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Primary, a)
    }
    pub fn Secondary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Secondary, a)
    }
    pub fn Ghost(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Ghost, a)
    }
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let colors = COLORS.button.colors_from(self.4, ButtonState::Default);
        let font = ctx.load_font("fonts/outfit_bold.ttf").unwrap(); // GET LABEL FONT
        // let image = ctx.load_image("icons/pfp.png").unwrap(); // GET DESIRED ICON

        let bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (text_size, height, _icon_size, x_padding) = match self.1 {
            Size::Medium => (16, 32, 16, 12),
            Size::Large => (20, 48, 24, 24)
        };

        let content = Row!(3, Vec2::new(0, 0), Align::Center, false,
            // Image(ShapeType::Rectangle(icon_size, 8), image),
            (Text::new(self.0, colors.label, text_size, font.clone()), false)
        ).build(ctx, bound);

        let width = match self.2 {
            Width::Hug => content.size(ctx).x + (x_padding * 2),
            Width::Expand => max_size.x,
        };

        Stack!(Vec2::new(0, 0), self.5,
            (Shape(ShapeType::Rectangle(width, height), colors.background, None), Vec2::new(0, 0)),
            (Shape(ShapeType::Rectangle(width, height), colors.outline, Some(200)), Vec2::new(0, 0)),
            (Text::new(self.0, colors.label, text_size, font.clone()), Vec2::new(x_padding, 0))
        ).build_children(ctx, max_size)
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

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Large,
    Medium,
}

// == Text Input -- //


pub enum InputStatus {
    Default,
    Hover,
    Focus,
    Filled,
    Error
}

pub struct TextInput {
    status: InputStatus,
    label: &'static str,
    help_text: Option<&'static str>,
    err_text: Option<&'static str>,
    icon: Option<&'static str>,
    value: &'static str,
    place_holder: &'static str,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            status: InputStatus::Default,
            label: "Label",
            help_text: None,
            err_text: None,
            icon: None,
            value: "",
            place_holder: "Placeholder"
        }
    }
}

impl ComponentBuilder for TextInput {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let text = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        
        // ctx.include_assets(include_assets!("./resources")); 
        // let image = ctx.load_image("images/profile.png").unwrap();

        let mut column: Vec<Box<dyn ComponentBuilder>> = vec![];

        column.push(Box::new(Text::new(self.label, COLORS.text.heading, 16, heading.clone())));

        let mut content: Vec<Box<dyn ComponentBuilder>> = vec![];

        if self.value.len() > 0 {
            content.push(Box::new(Text::new(self.value, COLORS.text.primary, 16, text.clone())));
        } else {
            content.push(Box::new(Text::new(self.place_holder, COLORS.text.secondary, 16, text.clone())));
        }

        if let Some(_icon) = &self.icon {
            content.push(Box::new(Expand(false, 1, COLORS.background.primary)));
            content.push(Box::new(Shape(ShapeType::Rectangle(32, 32), "ffffff", None)));
        }

        let outline_color = match self.status {
            InputStatus::Default => COLORS.outline.secondary,
            InputStatus::Focus => COLORS.outline.primary,
            InputStatus::Filled => COLORS.outline.secondary,
            InputStatus::Error => COLORS.status.danger,
            InputStatus::Hover => COLORS.outline.secondary,
        };


        let input_field = ConstrainedBox!(393, Stack!(Vec2::new(0, 0), Align::Center,
            (Shape(ShapeType::Rectangle(393, 48), outline_color, Some(200)), Vec2::new(0, 0)),
            (Row!(16, Vec2::new(0, 0), Align::Left, true, content), Vec2::new(0, 0))
        ));

        column.push(Box::new(input_field));

        if let Some(help) = &self.help_text {
            column.push(Box::new(Text::new(help, COLORS.text.secondary, 14, text.clone())));
        } 

        if let Some(err) = &self.err_text {
            column.push(Box::new(Text::new(err, COLORS.status.danger, 14, text.clone())));
        } 

        Column!(16, Vec2::new(0, 0), Align::Left, true, column).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

// == Keypad == //

pub struct NumericKeypad();

impl ComponentBuilder for NumericKeypad {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ConstrainedBox!(300, 
            Column!(16, Vec2::new(0, 0), Align::Center, false,
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("1", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("2", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("3", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("4", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("5", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("6", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("7", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("8", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("9", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost(".", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("0", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("<", Size::Large, Width::Expand, Some("back"), Align::Center), true)
                )
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}