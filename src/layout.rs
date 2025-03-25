use rust_on_rails::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub enum Size {
    #[default]
    Fit,
    Fill,
    Static(u32)
}

impl Size {
    pub fn get(&self, max_size: u32, items: Vec<Option<u32>>) -> u32 {
        match self {
            Self::Fit => items.into_iter().fold(None, |os, s| {
                s.map(|s| os.map(|os: u32| os.max(s)).unwrap_or(s)).or(os)
            }).unwrap_or(max_size),
            Self::Fill => max_size,
            Self::Static(s) => *s,
        }
    }

    pub fn size(&self, items: Vec<Option<u32>>) -> Option<u32> {
        match self {
            Self::Fit => items.into_iter().fold(None,
                |os, s| s.map(|s| os.map(|os: u32| os.max(s)).unwrap_or(s)).or(os)
            ),
            Self::Fill => None,
            Self::Static(s) => Some(*s)
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Offset {
    #[default]
    Start,
    Center,
    End,
    Static(i32)
}

impl Offset {
    pub fn get(&self, max_size: u32, item: u32) -> i32 {
        match self {
            Self::Start => 0,
            Self::Center => (max_size as i32 - item as i32) / 2,
            Self::End => max_size as i32 - item as i32,
            Self::Static(s) => *s,
        }
    }

    pub fn size(&self) -> Option<i32> {
        match self {
            Self::Start => Some(0),
            Self::Center => None,
            Self::End => None,
            Self::Static(s) => Some(*s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack(pub (Offset, Offset), pub (Size, Size));

impl Stack {
    pub fn get_size(size: &Size, items: Vec<Option<u32>>) -> Option<u32> {
        match size {
            Size::Fit => items.into_iter().fold(None,
                |os, s| s.map(|s| os.map(|os: u32| os.max(s)).unwrap_or(s)).or(os)
            ),
            Size::Fill => None,
            Size::Static(s) => Some(*s)
        }
    }
}

impl Layout for Stack {
    fn build(&self, _ctx: &mut Context, max_size: (u32, u32), items: Vec<(Option<u32>, Option<u32>)>) -> Vec<((i32, i32), (u32, u32))> {
        let (widths, heights): (Vec<Option<u32>>, Vec<Option<u32>>) = items.iter().cloned().unzip();
        let stack_size = (Self::get_size(&self.1.0, widths).unwrap_or(max_size.0), Self::get_size(&self.1.1, heights).unwrap_or(max_size.1));
        items.into_iter().map(|(w, h)| {
            let size = (w.unwrap_or(stack_size.0), h.unwrap_or(stack_size.1));
            let offset = (self.0.0.get(stack_size.0, size.0), self.0.1.get(stack_size.1, size.1));
            (offset, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<(Option<u32>, Option<u32>)>) -> (Option<u32>, Option<u32>) {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().unzip();
        (Self::get_size(&self.1.0, widths), Self::get_size(&self.1.1, heights))
    }
}


#[derive(Clone, Debug)]
pub struct Column(pub u32, pub Offset, pub Size);

impl Column {
    pub fn get_size(size: &Size, items: Vec<Option<u32>>) -> Option<u32> {
        match size {
            Size::Fit => items.into_iter().try_fold(0, |os, s| s.map(|s| os.max(s))),
            Size::Fill => None,
            Size::Static(s) => Some(*s)
        }
    }
}


impl Layout for Column {
    fn build(&self, _ctx: &mut Context, max_size: (u32, u32), items: Vec<(Option<u32>, Option<u32>)>) -> Vec<((i32, i32), (u32, u32))> {
        let (widths, heights): (Vec<Option<u32>>, Vec<Option<u32>>) = items.iter().cloned().unzip();

        let col_width = Self::get_size(&self.2, widths).unwrap_or(max_size.0);
        // let col_width = widths.into_iter().try_fold(0, |os, s| s.map(|s| os.max(s))).unwrap();

        let spacing = (heights.len()-1) as u32 *self.0;
        let (count, sized) = heights.iter().fold((0, 0), |(c, t), h| (c+h.is_none() as u32, t+h.unwrap_or(0)));
        let unsized_space = if count == 0 {0} else {(max_size.1-(spacing+sized)) / count};

        let mut offset = 0;
        items.into_iter().map(|(w, h)| {
            let size = (w.unwrap_or(col_width), h.unwrap_or(unsized_space));
            let offset_v = (self.1.get(col_width, size.0), offset as i32);
            offset = size.1+self.0;
            (offset_v, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<(Option<u32>, Option<u32>)>) -> (Option<u32>, Option<u32>) {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().unzip();
        let items = heights.len();
        (
            Self::get_size(&self.2, widths),
            Self::get_size(&Size::Fit, heights).map(|t| t+(self.0*(items-1) as u32))
        )
    }
}

#[derive(Clone, Debug)]
pub struct Row(pub u32, pub Offset, pub Size);

impl Row {
    pub fn get_size(size: &Size, items: Vec<Option<u32>>) -> Option<u32> {
        match size {
            Size::Fit => items.into_iter().try_fold(0, |os, s| s.map(|s| os.max(s))),
            Size::Fill => None,
            Size::Static(s) => Some(*s)
        }
    }
}

impl Layout for Row {
    fn build(&self, _ctx: &mut Context, max_size: (u32, u32), items: Vec<(Option<u32>, Option<u32>)>) -> Vec<((i32, i32), (u32, u32))> {
        let (widths, heights): (Vec<Option<u32>>, Vec<Option<u32>>) = items.iter().cloned().unzip();
        
        let row_height = Self::get_size(&self.2, heights).unwrap_or(max_size.1);
        // let row_height = heights.into_iter().try_fold(0, |os, s| s.map(|s| os.max(s))).unwrap();

        let spacing = (widths.len()-1) as u32 *self.0;
        let (count, sized) = widths.iter().fold((0, 0), |(c, t), w| (c+w.is_none() as u32, t+w.unwrap_or(0)));
        let unsized_space = if count == 0 {0} else {(max_size.0-(spacing+sized)) / count};

        let mut offset = 0;
        items.into_iter().map(|(w, h)| {
            let size = (w.unwrap_or(unsized_space), h.unwrap_or(row_height));
            let offset_v = (offset as i32, self.1.get(row_height, size.1));
            offset = size.0+self.0;
            (offset_v, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<(Option<u32>, Option<u32>)>) -> (Option<u32>, Option<u32>) {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().unzip();
        let items = widths.len();
        (
            Self::get_size(&Size::Fit, widths).map(|t| t+(self.0*(items-1) as u32)),
            Self::get_size(&self.2, heights)
        )
    }
}


// #[derive(Clone, Debug, Component)]
// pub struct Test(#[layout] Row, Text, Text, #[skip] #[ignore] u32, #[skip] i32);

// impl Test {
//     pub fn new(font: Font, _font2: Font) -> Self {
//         Test(
//             Row(10, Offset::Center, Size::Fit),
//             Text::new("HELLO HE", Color::from_hex("eb343a", 255), None, 48, 60, font.clone()),
//             Text::new("HELLO HL", Color::from_hex("eb343a", 255), None, 24, 30, font.clone()),
//             30, -30
//         )
//     }
// }

// impl Component for Test {
//     fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
//     fn children(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
//     fn layout(&self) -> &dyn Layout {&self.0}

//     fn on_tick(&mut self, ctx: &mut Context) {
//         println!("size: {:?}", self.size(ctx));
//     }

//     fn on_click(&mut self, _ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
//         println!("position: {:?}", position);
//         true
//     }
//     fn on_move(&mut self, _ctx: &mut Context, _position: Option<(u32, u32)>) -> bool {
//         true
//     }
// }


// pub struct MyApp;

// impl App for MyApp {
//     async fn new(ctx: &mut Context<'_>) -> BoxComponent {
//         println!("START APP");

//         let font = resources::Font::new(ctx, include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());
//         let font2 = resources::Font::new(ctx, include_bytes!("../assets/fonts/outfit_regular.ttf").to_vec());
//         let test = Test::new(font, font2);
//         // test.answer();
//         Box::new(test)
//     }
// }

// create_entry_points!(MyApp);
