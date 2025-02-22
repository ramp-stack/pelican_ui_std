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
    pub children: Vec<(Box<dyn ComponentBuilder>, bool)>,
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
    // pub background: &'static str,
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
    
        bound.w -= self.padding.x;
        bound.h -= self.padding.y;
        bound.x += self.padding.x;
        bound.y += self.padding.y;
    
        let mut total_fixed_height: u32 = 0;
        let mut expands_y_count: u32 = 0;
        let mut fixed_heights: Vec<u32> = vec![];
        let mut child_widths: Vec<u32> = vec![];
    
        // First pass: determine heights and track widths
        for (child, is_expands) in &self.children {
            if *is_expands {
                expands_y_count += 1;
                total_fixed_height += self.spacing as u32;
                child_widths.push(0); // Placeholder for expands case
            } else {
                let size = child.build(ctx, bound).size(ctx);
                let width = size.x as u32;
                let height = size.y as u32;
                fixed_heights.push(height);
                child_widths.push(width);
                total_fixed_height += height + self.spacing as u32;
            }
        }
    
        // Determine widest child
        let widest_child: u32 = *child_widths.iter().max().unwrap_or(&0);
    
        let available_height: u32 = if expands_y_count > 0 {
            ((bound.h as u32).saturating_sub(total_fixed_height)) / expands_y_count
        } else {
            0
        };
    
        let current_x = bound.x;
        let mut current_y = bound.y;
        let mut fixed_index = 0;
    
        self.children
            .iter()
            .enumerate()
            .map(|(_index, (child, is_expands))| {
                let mut size_bound = bound;
                size_bound.x = current_x;
                size_bound.y = current_y;
    
                if *is_expands {
                    size_bound.h = available_height;
                } else {
                    size_bound.h = fixed_heights[fixed_index];
                    fixed_index += 1;
                }
    
                let mut child = child.build(ctx, size_bound);
                let (width, height) = (child.size(ctx).x, child.size(ctx).y);
    
                if self.align == Align::Center {
                    let x_offset = (widest_child - width) / 2;
                    child.1.w -= x_offset;
                    child.1.x += x_offset;
                }
    
                current_y += height + self.spacing as u32;
    
                Box::new(child) as Box<dyn Drawable>
            })
            .collect()
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
            
            // Calculate offsets from alignment
            let (x_offset, y_offset) = match self.align {
                Align::Center => align_center(max_width, width, max_height, height),
                Align::Left => (align_left(offset.x + self.padding.x), vert_center(max_height, height)),
                Align::Right => (align_right(max_width, width, offset.x + self.padding.x), 0),
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

pub struct Row {
    pub children: Vec<(Box<dyn ComponentBuilder>, bool)>, 
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
}

impl ComponentBuilder for Row {

    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        
        // Adjust for padding
        bound.w -= self.padding.x;
        bound.x += self.padding.x;
        bound.h += self.padding.y;
        bound.y += self.padding.y;
    
        let mut total_fixed_width = 0; // Keep track of unavailable space
        let mut expands_x_count = 0; // Keep track of how many objects will need to expand
        let mut fixed_widths = vec![];
    
        for (child, is_expands) in &self.children {
            if *is_expands {
                expands_x_count += 1; // Account for another expansive child
                total_fixed_width += self.spacing; // Account for the spacing
            } else {
                let size = child.build(ctx, bound).size(ctx); // Build the object to get size
                fixed_widths.push(size.x); // Remember size for later
                total_fixed_width += size.x + self.spacing; // Account for size and spacing
            }
        }

        if expands_x_count == 0 && self.align == Align::Center {
            let x = bound.w - total_fixed_width;
            bound.w -= x / 2;
            bound.x += x / 2;
        }
    
        // Subtract fixed_width from available space, then divide by expansive objects
        let available_width = match expands_x_count > 0 {
            true => (bound.w - total_fixed_width).max(0) / expands_x_count,
            false => 0 // Match cause expands_x_count is 0 crashed
        };
    
        let mut current_x = bound.x;
        let current_y = bound.y;
        let mut fixed_index = 0;
    
        self.children
            .iter()
            .enumerate()
            .map(|(_index, (child, is_expands))| {
                let mut size_bound = bound;
                size_bound.x = current_x;
                size_bound.y = current_y;
    
                if *is_expands {
                    size_bound.w = available_width; // Space allocated for expansive
                } else {
                    size_bound.w = fixed_widths[fixed_index]; // Space previously remembered
                    fixed_index += 1; // Up the count of fixed objects
                }
    
                let child = child.build(ctx, size_bound); // Build object with new bounds
                let width = child.size(ctx).x; // Get final width
                current_x += width + self.spacing; // Update current_x to make row
    
                Box::new(child) as Box<dyn Drawable>
            })
            .collect()
    }    

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}