use rust_on_rails::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub enum Size {
    #[default]
    Fit,
    ExpandToFit,
    Fill,
    Static(u32, u32)
}

impl Size {
    pub fn get(&self, max_size: (u32, u32), min_size: (u32, u32)) -> (u32, u32) {
        match self {
            Self::Fit =>(min_size.0.min(max_size.0), min_size.1.min(max_size.1)),
            Self::ExpandToFit => min_size,
            Self::Fill => max_size,
            Self::Static(x, y) => (*x, *y)
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Offset {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Static(i32, i32)
}

impl Offset {
    pub fn get(&self, max_size: (u32, u32), min_size: (u32, u32)) -> (i32, i32) {
        match self {
            Self::TopLeft => (0, 0),
            Self::TopCenter => ((max_size.0 as i32 - min_size.0 as i32) / 2, 0),
            Self::TopRight => (max_size.0 as i32 - min_size.0 as i32, 0),
            Self::Left => (0, (max_size.1 as i32 - min_size.1 as i32) / 2),
            Self::Center => ((max_size.0 as i32 - min_size.0 as i32) / 2, (max_size.1 as i32 - min_size.1 as i32) / 2),
            Self::Right => (max_size.0 as i32 - min_size.0 as i32, (max_size.1 as i32 - min_size.1 as i32) / 2),
            Self::BottomLeft => (0, max_size.1 as i32 - min_size.1 as i32),
            Self::BottomCenter => (((max_size.0 as i32 - min_size.0 as i32) / 2), (max_size.1 as i32 - min_size.1 as i32)),
            Self::BottomRight => ((max_size.0 as i32 - min_size.0 as i32), (max_size.1 as i32 - min_size.1 as i32)),
            Self::Static(x, y) => (*x, *y)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack(pub Offset, pub Size);

impl Layout for Stack {
    fn layout(&self, ctx: &mut Context, max_size: (u32, u32), mut items: Vec<SizeFn>) -> Vec<((i32, i32), (u32, u32))> {
        let min_size = items.iter_mut().map(|i| i(ctx, max_size)).fold((0, 0), |old_size, size| {
            (old_size.0.max(size.0), old_size.1.max(size.1))
        });
        let max_size = self.1.get(max_size, min_size);
        
        items.into_iter().map(|mut i| {
            let size = i(ctx, max_size);
            let offset = self.0.get(max_size, size);
            (offset, size)
        }).collect()
    }
}

// Three variations (Fill, Fit, Fixed(x))

// Two dimensions (Width, Height)

// Size(Fill, Fill)
// Size(Fit, Fit)
// Size(Fixed(48), Fixed(48))
// Size(Fill, Fit)
// Size(Fill, Fixed(48))
// Size(Fixed(48), Fill)
// Size(Fixed(48), Fit)
// Size(Fit, Fixed(48))
// Size(Fit, Fill)

// Each 'child' has Size enum

// Calculate space allocated to Fit or Fixed(x) children
// Divide remaining space amongst Fill children

// If no Fill children, set parent/row/column/stack width to Fit (it's children)
// If has Fill children, set parent/row/column/stack width to Fill (it's parent)

// Button(Size(Fill, Fixed(48)), children);

// Row(RowOffset::Center, children);







pub struct Column(pub u32, pub ColumnOffset);

impl Layout for Column {
    fn layout(&self, ctx: &mut Context, max_size: (u32, u32), items: Vec<SizeFn>) -> Vec<((i32, i32), (u32, u32))> {
        let mut offset = 0;
        items.into_iter().map(|mut i| {
            let size = i(ctx, (max_size.0, max_size.1-offset));
            let v = ((self.1.get(max_size.0, size.0), offset as i32), size);
            offset += size.1+self.0;
            v
        }).collect()
    }
}

pub enum ColumnOffset {
    Left,
    Center,
    Right
}

impl ColumnOffset {
    pub fn get(&self, max_width: u32, width: u32) -> i32 {
        match self {
            ColumnOffset::Left => 0,
            ColumnOffset::Center => (max_width as i32 - width as i32) / 2,
            ColumnOffset::Right => max_width as i32 - width as i32
        }
    }
}

pub struct Row(pub u32, pub RowOffset);

impl Layout for Row {
    fn layout(&self, ctx: &mut Context, max_size: (u32, u32), items: Vec<SizeFn>) -> Vec<((i32, i32), (u32, u32))> {
        let mut offset = 0;
        items.into_iter().map(|mut i| {
            let size = i(ctx, (max_size.0-offset, max_size.1));
            let v = ((offset as i32, self.1.get(max_size.1, size.1)), size);
            offset += size.0+self.0;
            v
        }).collect()
    }
}

pub enum RowOffset {
    Top,
    Center,
    Bottom
}

impl RowOffset {
    pub fn get(&self, max_height: u32, height: u32) -> i32 {
        match self {
            RowOffset::Top => 0,
            RowOffset::Center => (max_height as i32 - height as i32) / 2,
            RowOffset::Bottom => max_height as i32 - height as i32
        }
    }
}

// #[macro_export]
// macro_rules! discard_nonsies {
//     ($($x:expr),* $(,)?) => {{
//         let mut v: Vec<Box<dyn ComponentTag>> = Vec::new();
//         $(
//             if let Some(value) = $x {
//                 v.push(value);
//             }
//         )*
//         v
//     }};
// }


// #[derive(Clone)]
// pub struct Padding(pub (u32, u32), pub Box<dyn ComponentTag>);

// impl Component for Padding {
//     fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
//         // println!("size: {} {}", max_size.0-(self.0.0*2), max_size.1-(self.0.1*2));
//         Container(Offset::Center, Size::Fill, vec![
//             Box::new(Container(
//                 Offset::default(), 
//                 Size::Static(max_size.0-(self.0.0*2), max_size.1-(self.0.1*2)),
//                 vec![self.1.clone()]
//             ))
//         ])
//     }
// }

// #[macro_export]
// macro_rules! Column {
//     ($x:expr, $( $child:expr ),* $(,)?) => {{
//         let children: Vec<Box<dyn ComponentTag>> = vec![
//             $( Box::new($child) as Box<dyn ComponentTag> ),*
//         ];
        
//         Column($x, children)
//     }};
// }

// #[derive(Clone)]
// pub struct Column(pub u32, pub Vec<Box<dyn ComponentTag>>);

// impl Component for Column {
//     fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
//         let mut offset = 0;
//         Container(Offset::default(), Size::Fill,
//             self.1.iter().flat_map(|c| {
//                 // println!("Column offset {:?}, w {:?}, h {:?}", offset, max_size.0, max_size.1);
//                 if max_size.1 < offset {
//                     // println!("Column item to big. Discarding."); 
//                     None 
//                 } else {
//                     let size = c.size(ctx, (max_size.0, max_size.1-offset));
//                     // println!("Column object size: {:?}", size);
//                     let v = Container(Offset::Static(0, offset as i32), Size::Fit, vec![c.clone()]);
//                     offset += size.1+self.0;
//                     Some(Box::new(v) as Box<dyn ComponentTag>)
//                 }
//             }).collect()
//         )
//     }

//     fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
//         let mut offset = 0;
//         for c in &mut self.1 {
//             let size = c.size(ctx, (max_size.0, max_size.1-offset));
//             if size.0 > position.0 && offset+size.1 > position.1 {
//                 c.on_click(
//                     ctx,
//                     (max_size.0, offset+self.0),
//                     (position.0, position.1-offset)
//                 );
//                 return;
//             }
//             if offset+size.1+self.0 > position.1 {
//                 return;
//             }
//             offset += self.0 + size.1;
//         }
//     }
// }


// #[macro_export]
// macro_rules! Row {
//     ($x:expr, $i:expr, $( $child:expr ),* $(,)?) => {{
//         let children: Vec<Box<dyn ComponentTag>> = vec![
//             $( Box::new($child) as Box<dyn ComponentTag> ),*
//         ];
        
//         Row($x, $i, children)
//     }};
// }

// #[derive(Clone)]
// pub struct Row(pub u32, pub Offset, pub Vec<Box<dyn ComponentTag>>);

// impl Component for Row {
//     fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
//         let mut offset = 0;
//         // println!("Row max size: {:?}", max_size);
//         Container(self.1, Size::Fit,
//             self.2.iter().flat_map(|c| {
//                 if max_size.0 < offset { 
//                     // println!("Row item to big. Discarding."); 
//                     None 
//                 } else {
//                     let size = c.size(ctx, (max_size.0-offset, max_size.1));
//                     // println!("Row Item size {:?}", size);
//                     let v = Container(Offset::Static(offset as i32, 0), Size::Fit, vec![c.clone()]);
//                     offset += size.0+self.0;
//                     Some(Box::new(v) as Box<dyn ComponentTag>)
//                 }
//             }).collect()
//         )
//     }

//     fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
//         let mut offset = 0;
//         for c in &mut self.2 {
//             let size = c.size(ctx, (max_size.0, max_size.1-offset));
//             if size.0 > position.0 && offset+size.1 > position.1 {
//                 c.on_click(
//                     ctx,
//                     (max_size.0, offset+self.0),
//                     (position.0, position.1-offset)
//                 );
//                 return;
//             }
//             if offset+size.1+self.0 > position.1 {
//                 return;
//             }
//             offset += self.0 + size.1;
//         }
//     }
// }