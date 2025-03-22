use rust_on_rails::prelude::*;

#[macro_export]
macro_rules! discard_nonsies {
    ($($x:expr),* $(,)?) => {
        {
            let mut v: Vec<Box<dyn ComponentTag>> = Vec::new();
            $(
                if let Some(value) = $x {
                    v.push(value);
                }
            )*
            v
        }
    };
}


#[derive(Clone)]
pub struct Padding(pub (u32, u32), pub Box<dyn ComponentTag>);

impl Component for Padding {
    fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
        // println!("size: {} {}", max_size.0-(self.0.0*2), max_size.1-(self.0.1*2));
        Container(Offset::Center, Size::Fill, vec![
            Box::new(Container(
                Offset::default(), 
                Size::Static(max_size.0-(self.0.0*2), max_size.1-(self.0.1*2)),
                vec![self.1.clone()]
            ))
        ])
    }
}

#[macro_export]
macro_rules! Column {
    ($x:expr, $( $child:expr ),* $(,)?) => {{
        let children: Vec<Box<dyn ComponentTag>> = vec![
            $( Box::new($child) as Box<dyn ComponentTag> ),*
        ];
        
        Column($x, children)
    }};
}

#[derive(Clone)]
pub struct Column(pub u32, pub Vec<Box<dyn ComponentTag>>);

impl Component for Column {
    fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
        let mut offset = 0;
        Container(Offset::default(), Size::Fill,
            self.1.iter().flat_map(|c| {
                // println!("Column offset {:?}, w {:?}, h {:?}", offset, max_size.0, max_size.1);
                if max_size.1 < offset {
                    // println!("Column item to big. Discarding."); 
                    None 
                } else {
                    let size = c.size(ctx, (max_size.0, max_size.1-offset));
                    // println!("Column object size: {:?}", size);
                    let v = Container(Offset::Static(0, offset as i32), Size::Fit, vec![c.clone()]);
                    offset += size.1+self.0;
                    Some(Box::new(v) as Box<dyn ComponentTag>)
                }
            }).collect()
        )
    }

    fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
        let mut offset = 0;
        for c in &mut self.1 {
            let size = c.size(ctx, (max_size.0, max_size.1-offset));
            if size.0 > position.0 && offset+size.1 > position.1 {
                c.on_click(
                    ctx,
                    (max_size.0, offset+self.0),
                    (position.0, position.1-offset)
                );
                return;
            }
            if offset+size.1+self.0 > position.1 {
                return;
            }
            offset += self.0 + size.1;
        }
    }
}


#[macro_export]
macro_rules! Row {
    ($x:expr, $i:expr, $( $child:expr ),* $(,)?) => {{
        let children: Vec<Box<dyn ComponentTag>> = vec![
            $( Box::new($child) as Box<dyn ComponentTag> ),*
        ];
        
        Row($x, $i, children)
    }};
}

#[derive(Clone)]
pub struct Row(pub u32, pub Offset, pub Vec<Box<dyn ComponentTag>>);

impl Component for Row {
    fn build(&self, ctx: &mut Context, mut max_size: (u32, u32)) -> Container {
        let mut offset = 0;
        // println!("Row max size: {:?}", max_size);
        Container(self.1, Size::Fit,
            self.2.iter().flat_map(|c| {
                if max_size.0 < offset { 
                    // println!("Row item to big. Discarding."); 
                    None 
                } else {
                    let size = c.size(ctx, (max_size.0-offset, max_size.1));
                    // println!("Row Item size {:?}", size);
                    let v = Container(Offset::Static(offset as i32, 0), Size::Fit, vec![c.clone()]);
                    offset += size.0+self.0;
                    Some(Box::new(v) as Box<dyn ComponentTag>)
                }
            }).collect()
        )
    }

    fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
        let mut offset = 0;
        for c in &mut self.2 {
            let size = c.size(ctx, (max_size.0, max_size.1-offset));
            if size.0 > position.0 && offset+size.1 > position.1 {
                c.on_click(
                    ctx,
                    (max_size.0, offset+self.0),
                    (position.0, position.1-offset)
                );
                return;
            }
            if offset+size.1+self.0 > position.1 {
                return;
            }
            offset += self.0 + size.1;
        }
    }
}