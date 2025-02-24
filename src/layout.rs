use rust_on_rails::prelude::*;
use crate::{Align};

fn horz_center(max_width: u32, width: u32) -> u32 { (max_width - width) / 2 }
fn vert_center(max_height: u32, height: u32) -> u32 { (max_height - height) / 2 }
fn align_center(mw: u32, w: u32, mh: u32, h: u32) -> (u32, u32) { (horz_center(mw, w), vert_center(mh, h)) }
fn align_right(max_width: u32, width: u32) -> u32 { max_width - width }
fn align_bottom(max_height: u32, height: u32, padding: u32) -> u32 { (max_height - height) - padding }
fn align_top(padding: u32) -> u32 { padding }
fn align_left(padding: u32) -> u32 { padding }

pub struct Column {
    pub children: Vec<(Box<dyn ComponentBuilder>, bool)>,
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let bound = Rect::new(0, self.padding.y, max_size.x, max_size.y - (self.padding.y * 2));
        let (mut heights, mut fixed_height, mut expands_count, mut max_width) = (Vec::new(), 0, 0, 0);
    
        for (child_builder, expands) in &self.children {
            let child = child_builder.build(ctx, bound);
            let (height, width) = (child.size(ctx).y, child.size(ctx).x);

            if width > max_width { max_width = width; }

            match *expands {
                true => {expands_count += 1; fixed_height += self.spacing;}
                false => {fixed_height += height + self.spacing; heights.push(height);}
            }
        }
    
        let allocated = bound.w.saturating_sub(fixed_height);
        let expand_height = if expands_count > 0 { allocated / expands_count } else { 0 };
    
        let (mut current_y, mut slot) = (bound.y, 0);
        let mut final_children: Vec<Box<dyn Drawable>> = Vec::new();

        for (builder, expands) in &self.children {
            let mut bound = bound;
            bound.y = current_y;
            if *expands { bound.h = expand_height; } else { bound.h = heights[slot]; slot += 1; }

            let mut child = builder.build(ctx, bound);
            let (height, width) = (child.size(ctx).y, child.size(ctx).x);

            let x_offset = match self.align {
                Align::Right => align_right(max_width, width),
                _ => 0
            };

            child.1.w -= x_offset;
            child.1.x += x_offset;

            current_y += height + self.spacing;
            final_children.push(Box::new(child));
        }
        
        // let mut end_padding_bound = Rect::new(bound.w, current_y, bound.w, self.padding.y);
        // final_children.push(Box::new(Padding(Vec2::new(self.padding.y, 0), "000000").build(ctx, end_padding_bound)));

        final_children
    }
    

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Row {
    pub children: Vec<(Box<dyn ComponentBuilder>, bool)>, 
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
}

impl ComponentBuilder for Row {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
        let (mut widths, mut fixed_width, mut expands_count, mut max_height) = (Vec::new(), 0, 0, 0);
    
        for (child_builder, expands) in &self.children {
            let child = child_builder.build(ctx, bound);
            let width = child.size(ctx).x;
            let height = child.size(ctx).y;

            if height > max_height { max_height = height; }

            match *expands {
                true => {expands_count += 1; fixed_width += self.spacing;}
                false => {fixed_width += width + self.spacing; widths.push(width);}
            }
        }
    
        let allocated = bound.w.saturating_sub(fixed_width);
        let expand_width = if expands_count > 0 { allocated / expands_count } else { 0 };
    
        let (mut current_x, mut slot) = (bound.x, 0);
        let mut final_children: Vec<Box<dyn Drawable>> = Vec::new();

        for (builder, expands) in &self.children {
            let mut bound = bound;
            bound.x = current_x;

            if *expands { bound.w = expand_width; } else { bound.w = widths[slot]; slot += 1; }

            let mut child = builder.build(ctx, bound);
            let (height, width) = (child.size(ctx).y, child.size(ctx).x);

            let y_offset = match self.align {
                Align::Center => vert_center(max_height, height),
                _ => 0
            };

            child.1.h -= y_offset;
            child.1.y += y_offset;

            current_x += width + self.spacing;
            final_children.push(Box::new(child));
        }
        
        // let mut end_padding_bound = Rect::new(current_x, 0, self.padding.x, 0);
        // final_children.push(Box::new(Padding(Vec2::new(self.padding.x, 0), "000000").build(ctx, end_padding_bound)));

        final_children
    }
    
    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
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

        // Get height/width of tallest/widest object
        let (max_width, max_height) = self.children.iter()
            .map(|(builder, _)| builder.build(ctx, bound).size(ctx))
            .fold((0, 0), |(max_x, max_y), size| (
                max_x.max(size.x), max_y.max(size.y)
            ));

        // Adjust bounds for paddings
        bound.h -= self.padding.y;
        bound.y += self.padding.y;
        bound.w -= self.padding.x;
        bound.x += self.padding.x;

        self.children.iter().map(|(builder, offset)| {
            // Build child and grab width/height
            let mut child = builder.build(ctx, bound);
            let (width, height) = (child.size(ctx).x, child.size(ctx).y);
            
            // Get offsets from alignment
            let (x_offset, y_offset) = match self.align {
                Align::Center => align_center(max_width, width, max_height, height),
                Align::Left => (align_left(offset.x + self.padding.x), vert_center(max_height, height)),
                Align::Right => (align_right(max_width, width), 0),
                Align::Top => (horz_center(max_width, width), align_top(offset.y + self.padding.y)),
                Align::Bottom => (horz_center(max_width, width), align_bottom(max_height, height, offset.y + self.padding.y))
            };

            // Adjust for offsets
            child.1.h -= y_offset;
            child.1.y += y_offset;
            child.1.w -= x_offset;
            child.1.x += x_offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()

    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}