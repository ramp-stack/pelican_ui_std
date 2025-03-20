use rust_on_rails::prelude::*;

#[derive(Clone, Copy)]
pub enum Align {
    TopLeft,
    TopCenter,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Align {
    pub fn align(&self, max_size: Vec2, min_size: Vec2) -> Vec2 {
        match self {
            Align::TopLeft => Vec2::new(0, 0),
            Align::TopCenter => Vec2::new((max_size.x - min_size.x) / 2, 0),
            Align::TopRight => Vec2::new((max_size.x - min_size.x), 0),
            Align::Left => Vec2::new(0, (max_size.y - min_size.y) / 2),
            Align::Center => Vec2::new((max_size.x - min_size.x) / 2, (max_size.y - min_size.y) / 2),
            Align::Right => Vec2::new((max_size.x - min_size.x), (max_size.y - min_size.y) / 2),
            Align::BottomLeft => Vec2::new(0, (max_size.y - min_size.y)),
            Align::BottomCenter => Vec2::new((max_size.x - min_size.x) / 2, (max_size.y - min_size.y)),
            Align::BottomRight => Vec2::new((max_size.x - min_size.x), (max_size.y - min_size.y))
        }
    }
}

#[macro_export]
macro_rules! Column {
    ($x:expr, $i:expr, $a:expr, $children:expr) => {{
        let children: Vec<Box<dyn ComponentBuilder>> = $children
            .into_iter()
            .map(|child| Box::new(child) as Box<dyn ComponentBuilder>)
            .collect();

        Column { 
            children,
            spacing: $i,
            align: $a,
            padding: $x
        }
    }};
}

pub struct Column {
    pub children: Vec<Box<dyn ComponentBuilder>>, 
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
        let (mut max_width, mut current_y) = (0, 0);
    
        for builder in &self.children {
            let child = builder.build(ctx, bound);
            let width = child.size(ctx).x;

            if width > max_width { max_width = width; }
        }

        self.children.iter().map(|child| {
            let mut bound = bound;
            bound.y = current_y;

            let mut child = child.build(ctx, bound);
            let (height, width) = (child.size(ctx).y, child.size(ctx).x);

            let y_offset = 0;

            child.1.w -= y_offset;
            child.1.x += y_offset;

            current_y += height + self.spacing;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }
}

#[macro_export]
macro_rules! Row {
    ($x:expr, $i:expr, $a:expr, $children:expr) => {{
        let children: Vec<Box<dyn ComponentBuilder>> = $children
            .into_iter()
            .map(|child| Box::new(child) as Box<dyn ComponentBuilder>)
            .collect();

        Row { 
            children,
            spacing: $i,
            align: $a,
            padding: $x
        }
    }};
}

pub struct Row {
    pub children: Vec<Box<dyn ComponentBuilder>>, 
    pub spacing: u32,
    pub align: Align,
    pub padding: Vec2,
}

impl ComponentBuilder for Row {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
        let (mut max_height, mut current_x) = (0, 0);
    
        for builder in &self.children {
            let child = builder.build(ctx, bound);
            let width = child.size(ctx).x;
            let height = child.size(ctx).y;

            if height > max_height { max_height = height; }
        }

        self.children.iter().map(|child| {
            let mut bound = bound;
            bound.x = current_x;

            let mut child = child.build(ctx, bound);
            let (height, width) = (child.size(ctx).y, child.size(ctx).x);

            let y_offset = 0;

            child.1.h -= y_offset;
            child.1.y += y_offset;

            current_x += width + self.spacing;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }
}


// pub struct Row {
//     pub children: Vec<(Box<dyn ComponentBuilder>, bool)>, 
//     pub spacing: u32,
//     pub align: Align,
//     pub padding: Vec2,
// }

// impl ComponentBuilder for Row {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
//         let (mut widths, mut fixed_width, mut expands_count, mut max_height) = (Vec::new(), 0, 0, 0);
    
//         for (child_builder, expands) in &self.children {
//             let child = child_builder.build(ctx, bound);
//             let width = child.size(ctx).x;
//             let height = child.size(ctx).y;

//             if height > max_height { max_height = height; }

//             match *expands {
//                 true => {expands_count += 1; fixed_width += self.spacing;}
//                 false => {fixed_width += width + self.spacing; widths.push(width);}
//             }
//         }
    
//         let allocated = bound.w.saturating_sub(fixed_width);
//         let expand_width = if expands_count > 0 { allocated / expands_count } else { 0 };
    
//         let (mut current_x, mut slot) = (bound.x, 0);
//         let mut final_children: Vec<Box<dyn Drawable>> = Vec::new();

//         for (builder, expands) in &self.children {
//             let mut bound = bound;
//             bound.x = current_x;

//             if *expands { bound.w = expand_width; } else { bound.w = widths[slot]; slot += 1; }

//             let mut child = builder.build(ctx, bound);
//             let (height, width) = (child.size(ctx).y, child.size(ctx).x);

//             let y_offset = match self.align {
//                 Align::Center => vert_center(max_height, height),
//                 _ => 0
//             };

//             child.1.h -= y_offset;
//             child.1.y += y_offset;

//             current_x += width + self.spacing;
//             final_children.push(Box::new(child));
//         }
        
//         // let mut end_padding_bound = Rect::new(current_x, 0, self.padding.x, 0);
//         // final_children.push(Box::new(Padding(Vec2::new(self.padding.x, 0), "000000").build(ctx, end_padding_bound)));

//         final_children
//     }
    
//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct Stack {
//     pub children: Vec<(Box<dyn ComponentBuilder>, Vec2)>,
//     pub align: Align,
//     pub padding: Vec2
// }


#[macro_export]
macro_rules! Stack {
    ($x:expr, $($child:expr),* $(,)?) => {{
        let children: Vec<Box<dyn ComponentBuilder>> = vec![
            $(Box::new($child) as Box<dyn ComponentBuilder>),*
        ];
        
        Stack($x, children)
    }};
}

pub struct Stack(pub Align, pub Vec<Box<dyn ComponentBuilder>>);

impl ComponentBuilder for Stack {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        // Get height/width of tallest/widest object
        let (max_width, max_height, built) = self.1.iter()
            .map(|builder| {
                let built = builder.build(ctx, bound);
                let size = built.size(ctx);
                (size, Box::new(built) as Box<dyn Drawable>)
            })
            .fold((0, 0, Vec::new()), |(max_x, max_y, mut built_vec), (size, built)| {
                built_vec.push(built);
                (max_x.max(size.x), max_y.max(size.y), built_vec)
            });

        // let built = self.1.iter()
        //     .map(|builder| {
        //         let a = builder.build(ctx, bound);
        //         // println!("BEFORE RESIZE");
        //         // let size = a.size(ctx);
        //         Box::new(a) as Box<dyn Drawable>
        //     }).collect();

                    // Get offsets from alignment
        // let (x_offset, y_offset) = match self.1 {
        //     Align::Center => align_center(max_width, width, max_height, height),
        //     _ => align_center(max_width, width, max_height, height),
        //     // Align::Left => (align_left(offset.x + self.0.x), vert_center(max_height, height)),
        //     // Align::Right => (align_right(max_width, width), 0),
        //     // Align::Top => (horz_center(max_width, width), align_top(offset.y + self.0.y)),
        //     // Align::Bottom => (horz_center(max_width, width), align_bottom(max_height, height, offset.y + self.0.y))
        // };


        self.1.iter().map(|builder| {
            // Build child and grab width/height
            let mut child = builder.build(ctx, bound);
            let (width, height) = (child.size(ctx).x, child.size(ctx).y);
            


            let offsets = Vec2::new(0, 0,); // align_center(max_width, width, max_height, height);

            // Adjust for offsets
            child.1.h -= offsets.y;
            child.1.y += offsets.y;
            child.1.w -= offsets.x;
            child.1.x += offsets.x;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

pub struct Container(pub Align, pub Box<dyn ComponentBuilder>);

impl ComponentBuilder for Container {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        let mut component = self.1.build(ctx, bound);
        let size = component.size(ctx);

        let offsets = self.0.align(max_size, size);

        component.1.h -= offsets.y;
        component.1.y += offsets.y;
        component.1.w -= offsets.x;
        component.1.x += offsets.x;

        vec![Box::new(component)]
    }
}