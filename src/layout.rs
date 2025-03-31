use rust_on_rails::prelude::*;

#[derive(Clone, Debug)]
pub struct Row(pub u32, pub Offset, pub Size);

impl Row {
    pub fn center(spacing: u32) -> Self {
        Row(spacing, Offset::Center, Size::Fit)
    }
    fn fit_width(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0+i.0, s.1+i.1)).unwrap_or_default()
    }

    fn fit_height(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }
}


impl Layout for Row {
    fn build(&self, _ctx: &mut Context, row_size: (u32, u32), items: Vec<SizeInfo>) -> Vec<((i32, i32), (u32, u32))> {
        let mut sizes: Vec<_> = items.iter().map(|i| {
            let size = i.get((i.min_width().0, row_size.1));
            (size.0 as f32, size.1)
        }).collect();

        let padding = (items.len()-1) as u32 * self.0;
        let min_width = items.iter().fold(MinSize::default(), |s, i| s+i.min_width())+padding;

        let mut free_space = (row_size.0 as i32 - min_width.0 as i32).max(0) as f32;
        while free_space > 0.0 {
            let (min_exp, count, next) = items.iter().zip(sizes.iter()).fold((None, 0.0, free_space as f32), |(mut me, mut c, mut ne), (i, size)| {
                let width = size.0;
                let max_width = i.max_width().0 as f32;
                if width < max_width { //I can expand
                    match me {
                        Some(w) if w < width => {
                            println!("w: {:?}, width: {:?}", w, width);
                            ne = ne.min(width-w);//Next size could be the min size of next expandable block
                        },
                        Some(w) if w == width => {
                            ne = ne.min(max_width-width);//Next size could be the max size of one of the smallest items
                            c += 1.0;
                        },
                        Some(w) if w > width => {
                            ne = ne.min(max_width-width).min(w-width);//Next size could be the max size of one of the smallest items
                            me = Some(width);
                            c = 1.0;
                        },
                        _ => {
                            ne = ne.min(max_width-width);//Next size could be the max size of one of the smallest items
                            me = Some(width);
                            c = 1.0;
                        }
                    }
                }
                (me, c, ne)
            });
            println!("ne: {:?}", next);
            if min_exp.is_none() {break;}
            let min_exp = min_exp.unwrap();

            println!("finish");
            let expand = (next*count).min(free_space);//Next size could be the rest of the free_space
            free_space -= expand;
            let expand = expand / count;

            sizes.iter_mut().zip(items.iter()).for_each(|(size, i)| {
                if size.0 < i.max_width().0 as f32 && size.0 == min_exp {
                    size.0 += expand;
                }
            });
        }

        let mut offset = 0;
        sizes.into_iter().map(|size| {
            let width = size.0.floor() as u32;
            let v = ((offset as i32, self.1.get(row_size.1, size.1)), (width, size.1));
            offset += width+self.0;
            v
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<SizeInfo>) -> SizeInfo {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let items = widths.len() as u32;
        let padding = self.0*(items-1);
        let width = Row::fit_width(widths);
        let height = self.2.get().unwrap_or_else(|| Row::fit_height(heights));
        let min_width = width.0 + padding;
        let max_width = width.1 + padding;
        SizeInfo::new(min_width, height.0, max_width, height.1)
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
            Self::Static(s) => *s as i32,
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

#[derive(Clone, Copy, Default, Debug)]
pub enum Size {
    #[default]
    Fit,
    Fill(MinSize, MaxSize),
    Static(u32)
}

impl Size {
    pub fn get(&self) -> Option<(MinSize, MaxSize)> {
        match self {
            Size::Fit => None,
            Size::Fill(min, max) => Some((*min, *max)),
            Size::Static(s) => Some((MinSize(*s), MaxSize(*s)))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Stack(pub Offset, pub Offset, pub Size, pub Size);
impl Stack {
    pub fn center() -> Self {
        Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit)
    }
    fn fit(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }
}

impl Layout for Stack {
    fn build(&self, _ctx: &mut Context, stack_size: (u32, u32), items: Vec<SizeInfo>) -> Vec<((i32, i32), (u32, u32))> {
        items.into_iter().map(|i| {
            let size = i.get(stack_size);
            let offset = (self.0.get(stack_size.0, size.0), self.1.get(stack_size.1, size.1));
            (offset, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<SizeInfo>) -> SizeInfo {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let size = (
            self.2.get().unwrap_or_else(|| Stack::fit(widths)),
            self.3.get().unwrap_or_else(|| Stack::fit(heights))
        );
        SizeInfo::new(size.0.0, size.1.0, size.0.1, size.1.1)
    }
}

#[derive(Debug, Clone, Component)]
pub struct Padding<D: Drawable + Clone>(Stack, D);
impl<D: Drawable + Clone> Events for Padding<D> {}

impl<D: Drawable + Clone> Padding<D> {
    pub fn new(ctx: &mut Context, item: D, padding: (u32, u32, u32, u32)) -> Self {
        let size = item.size(ctx);
        let wp = padding.0+padding.2;
        let hp = padding.1+padding.3;
        Padding(Stack(
            Offset::Static(padding.0 as i32), Offset::Static(padding.1 as i32), 
            Size::Fill(size.min_width()+MinSize(padding.2), size.max_width()-MaxSize(padding.2)),
            Size::Fill(size.min_height()+MinSize(padding.3), size.max_height()-MaxSize(padding.3))
        ), item)
    }
}