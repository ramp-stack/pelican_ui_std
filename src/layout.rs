use rust_on_rails::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Align {
    Left,
    Right,
    Center,
    Bottom,
    Top,
}

fn get_max_width(
    comp: &Vec<Box<dyn ComponentBuilder>>, 
    ctx: &mut ComponentContext, 
    bound: Rect
) -> u32 {
    comp
        .iter()
        .map(|builder| builder.build(ctx, bound).size(ctx).x)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0)
}

fn get_max_height(
    comp: &Vec<Box<dyn ComponentBuilder>>, 
    ctx: &mut ComponentContext, 
    bound: Rect
) -> u32 {
    comp
        .iter()
        .map(|builder| builder.build(ctx, bound).size(ctx).y)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0)
}


fn horz_center(max_width: u32, width: u32) -> u32 { (max_width - width) / 2 }
fn vert_center(max_height: u32, height: u32) -> u32 { (max_height - height) / 2 }
fn align_center(mw: u32, w: u32, mh: u32, h: u32) -> (u32, u32) { (horz_center(mw, w), vert_center(mh, h)) }
fn align_right(max_width: u32, width: u32, padding: u32) -> u32 { (max_width - width) - padding }
fn align_bottom(max_height: u32, height: u32, padding: u32) -> u32 { (max_height - height) - padding }
fn align_top(padding: u32) -> u32 { padding }
fn align_left(padding: u32) -> u32 { padding }

pub struct Column {
    pub children: Vec<Box<dyn ComponentBuilder>>,
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        let max_width = get_max_width(&self.children, ctx, bound);

        self.children.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let (width, height) = (child.size(ctx).x, child.size(ctx).y);

            let x_offset = match self.align {
                Align::Right => align_right(max_width, width, self.padding.x),
                Align::Left => align_left(self.padding.x),
                Align::Center => horz_center(max_width, width),
                _ => 0
            };

            bound.h -= self.spacing + height;
            bound.y += self.spacing + height;

            child.1.h -= self.padding.y;
            child.1.y += self.padding.y;
            child.1.w -= x_offset;
            child.1.x += x_offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Stack {
    pub children: Vec<(Box<dyn ComponentBuilder>, Vec2)>,
    pub align: Align,
    pub padding: Vec2
}

impl ComponentBuilder for Stack {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (max_width, max_height) = self.children.iter()
            .map(|(builder, _)| builder.build(ctx, bound).size(ctx))
            .fold((0, 0), |(max_x, max_y), size| (
                max_x.max(size.x), max_y.max(size.y)
            ));
            
        self.children.iter().map(|(builder, offset)| {
            let mut child = builder.build(ctx, bound);
            let (width, height) = (child.size(ctx).x, child.size(ctx).y);
            
            let (x_offset, y_offset) = match self.align {
                Align::Center => align_center(max_width, width, max_height, height),
                Align::Left => (align_left(offset.x + self.padding.x), 0),
                Align::Right => (align_right(max_width, width, offset.x + self.padding.x), 0),
                Align::Top => (0, align_top(offset.y + self.padding.y)),
                Align::Bottom => (0, align_bottom(max_height, height, offset.y + self.padding.y))
            };

            child.1.h -= y_offset;
            child.1.y += y_offset;
            child.1.w -= x_offset;
            child.1.x += x_offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()

    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Row {
    pub children: Vec<Box<dyn ComponentBuilder>>,
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2
}

impl ComponentBuilder for Row {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        let max_height = get_max_height(&self.children, ctx, bound);

        self.children.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let (width, height) = (child.size(ctx).x, child.size(ctx).y);

            let y_offset = match self.align {
                Align::Bottom => align_bottom(max_height, height, self.padding.y),
                Align::Top => align_top(self.padding.y),
                Align::Center => vert_center(max_height, height),
                _ => 0
            };

            bound.w -= self.spacing + width;
            bound.x += self.spacing + width;

            child.1.w -= self.padding.x;
            child.1.x += self.padding.x;
            child.1.h -= y_offset;
            child.1.y += y_offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

