use rust_on_rails::prelude::*;

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
pub struct Padding(pub u32, pub u32, pub u32, pub u32);

impl Padding {
    pub fn new(p: u32) -> Self {Padding(p, p, p, p)}

    fn adjust_size(&self, size: (u32, u32)) -> (u32, u32) {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        (size.0-wp, size.1-hp)
    }

    fn adjust_offset(&self, offset: (i32, i32)) -> (i32, i32) {
        (offset.0+self.0 as i32, offset.1+self.1 as i32)
    }

    fn adjust_info(&self, info: SizeInfo) -> SizeInfo {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        SizeInfo::new(info.min_width()+wp, info.min_height()+hp, info.max_width()+wp, info.max_height()+hp)
    }
}

pub struct UniformExpand;
impl UniformExpand {
    pub fn get(sizes: Vec<(u32, u32)>, max_size: u32, spacing: u32) -> Vec<u32> {
        let spacing = (sizes.len()-1) as u32 * spacing;
        let min_size = sizes.iter().fold(0, |s, i| s+i.0)+spacing;

        let mut sizes = sizes.into_iter().map(|s| (s.0 as f32, s.1)).collect::<Vec<_>>();

        let mut free_space = (max_size as i32 - min_size as i32).max(0) as f32;
        while free_space > 0.0 {
            let (min_exp, count, next) = sizes.iter().fold((None, 0.0, free_space as f32), |(mut me, mut c, mut ne), size| {
                let min = size.0 as f32;
                let max = size.1 as f32;
                if min < max { //I can expand
                    match me {
                        Some(w) if w < min => {
                            ne = ne.min(min-w);//Next size could be the min size of next expandable block
                        },
                        Some(w) if w == min => {
                            ne = ne.min(max-min);//Next size could be the max size of one of the smallest items
                            c += 1.0;
                        },
                        Some(w) if w > min => {
                            ne = ne.min(max-min).min(w-min);//Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        },
                        _ => {
                            ne = ne.min(max-min);//Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        }
                    }
                }
                (me, c, ne)
            });
            if min_exp.is_none() {break;}
            let min_exp = min_exp.unwrap();

            let expand = (next*count).min(free_space);//Next size could be the rest of the free_space
            free_space -= expand;
            let expand = expand / count;

            sizes.iter_mut().for_each(|size| {
                if size.0 < size.1 as f32 && size.0 == min_exp {
                    size.0 += expand;
                }
            });
        }
        sizes.into_iter().map(|s| s.0.floor() as u32).collect()
    }
}


#[derive(Clone, Debug)]
pub struct Row(pub u32, pub Offset, pub Size, pub Padding);

impl Row {
    pub fn center(spacing: u32) -> Self {
        Row(spacing, Offset::Center, Size::Fit, Padding::default())
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
        let row_size = self.3.adjust_size(row_size);

        let widths = UniformExpand::get(items.iter().map(|i| (i.min_width().0, i.max_width().0)).collect::<Vec<_>>(), row_size.0, self.0);

        let mut offset = 0;
        items.into_iter().zip(widths.into_iter()).map(|(i, width)| {
            let size = i.get((width, row_size.1));
            let off = self.3.adjust_offset((offset as i32, self.1.get(row_size.1, size.1)));
            offset += size.0+self.0;
            (off, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<SizeInfo>) -> SizeInfo {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(widths.len() as u32-1);
        let width = Self::fit_width(widths);
        let height = self.2.get().unwrap_or_else(|| Self::fit_height(heights));
        self.3.adjust_info(SizeInfo::new(width.0+spacing, height.0, width.1+spacing, height.1))
    }
}

#[derive(Clone, Debug)]
pub struct Column(pub u32, pub Offset, pub Size, pub Padding);

impl Column {
    pub fn center(spacing: u32) -> Self {
        Column(spacing, Offset::Center, Size::Fit, Padding::default())
    }
    fn fit_width(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }

    fn fit_height(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0+i.0, s.1+i.1)).unwrap_or_default()
    }
}


impl Layout for Column {
    fn build(&self, _ctx: &mut Context, col_size: (u32, u32), items: Vec<SizeInfo>) -> Vec<((i32, i32), (u32, u32))> {
        let col_size = self.3.adjust_size(col_size);

        let heights = UniformExpand::get(items.iter().map(|i| (i.min_height().0, i.max_height().0)).collect::<Vec<_>>(), col_size.1, self.0);

        let mut offset = 0;
        items.into_iter().zip(heights.into_iter()).map(|(i, height)| {
            let size = i.get((col_size.0, height));
            let off = self.3.adjust_offset((self.1.get(col_size.0, size.0), offset as i32));
            offset += size.1+self.0;
            (off, size)
        }).collect()
    }

    fn size(&self, _ctx: &mut Context, items: Vec<SizeInfo>) -> SizeInfo {
        let (widths, heights): (Vec<_>, Vec<_>) = items.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(heights.len() as u32-1);
        let width = self.2.get().unwrap_or_else(|| Self::fit_width(widths));
        let height = Self::fit_height(heights);
        self.3.adjust_info(SizeInfo::new(width.0, height.0+spacing, width.1, height.1+spacing))
    }
}




#[derive(Clone, Debug, Default)]
pub struct Stack(pub Offset, pub Offset, pub Size, pub Size, pub Padding);
impl Stack {
    pub fn center() -> Self {
        Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default())
    }
    fn fit(items: Vec<(MinSize, MaxSize)>) -> (MinSize, MaxSize) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }
}

impl Layout for Stack {
    fn build(&self, _ctx: &mut Context, stack_size: (u32, u32), items: Vec<SizeInfo>) -> Vec<((i32, i32), (u32, u32))> {
        let stack_size = self.4.adjust_size(stack_size);
        items.into_iter().map(|i| {
            let size = i.get(stack_size);
            let offset = (self.0.get(stack_size.0, size.0), self.1.get(stack_size.1, size.1));
            (self.4.adjust_offset(offset), size)
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
        self.4.adjust_info(SizeInfo::new(size.0.0, size.1.0, size.0.1, size.1.1))
    }
}

#[derive(Debug, Clone, Component)]
pub struct Bin<L: Layout + Clone, D: Drawable + Clone>(pub L, pub D);
impl<L: Layout + Clone, D: Drawable + Clone> Events for Bin<L, D> {}

#[derive(Debug, Clone, Component)]
pub struct Opt<D: Drawable + Clone + 'static>(Stack, Option<D>, #[skip] Option<D>);
impl<D: Drawable + Clone + 'static> Events for Opt<D> {}

impl<D: Drawable + Clone + 'static> Opt<D> {
    pub fn new(item: D, display: bool) -> Self {
        match display {
            true => Opt(Stack::default(), Some(item), None),
            false => Opt(Stack::default(), None, Some(item)),
        }
    }

    pub fn display(&mut self, display: bool) {
        match display {
            true if self.1.is_none() => self.1 = self.2.take(),
            false if self.2.is_none() => self.2 = self.1.take(),
            _ => {}
        }
    }

    pub fn inner(&mut self) -> &mut D {self.1.as_mut().unwrap_or_else(|| self.2.as_mut().unwrap())}
}

#[derive(Debug, Clone, Component)]
pub struct EitherOr<L: Drawable + Clone + 'static, R: Drawable + Clone + 'static>(Stack, Opt<L>, Opt<R>);
impl<L: Drawable + Clone + 'static, R: Drawable + Clone + 'static> Events for EitherOr<L, R> {}

impl<L: Drawable + Clone + 'static, R: Drawable + Clone + 'static> EitherOr<L, R> {
    pub fn new(left: L, right: R) -> Self {
        EitherOr(Stack::default(), Opt::new(left, true), Opt::new(right, false))
    }

    pub fn display_left(&mut self, display_left: bool) {
        self.1.display(display_left);
        self.2.display(!display_left);
    }

    pub fn left(&mut self) -> &mut L {self.1.inner()}
    pub fn right(&mut self) -> &mut R {self.2.inner()}
}