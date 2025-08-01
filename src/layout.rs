use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Layout, Area, SizeRequest};
use pelican_ui::{Context, Component};

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum Offset {
    #[default]
    Start,
    Center,
    End,
    Static(f32)
}

impl Offset {
    pub fn get(&self, max_size: f32, item_size: f32) -> f32 {
        match self {
            Self::Start => 0.0,
            Self::Center => (max_size - item_size) / 2.0,
            Self::End => max_size - item_size,
            Self::Static(offset) => *offset,
        }
    }

    pub fn size(&self) -> Option<f32> {
        match self {
            Self::Start => Some(0.0),
            Self::Center | Self::End => None,
            Self::Static(offset) => Some(*offset),
        }
    }
}

type CustomFunc = dyn Fn(Vec<(f32, f32)>) -> (f32, f32);
type FitFunc = fn(Vec<(f32, f32)>) -> (f32, f32);

#[derive(Default)]
pub enum Size {
    #[default]
    Fit,
    Fill(f32, f32),
    Static(f32),
    Custom(Box<CustomFunc>),
}

impl Size {
    pub fn fill() -> Self {Size::Fill(0.0, f32::MAX)}
    pub fn custom(func: impl Fn(Vec<(f32, f32)>) -> (f32, f32) + 'static) -> Self {
        Size::Custom(Box::new(func))
    }

    pub fn get(&self, items: Vec<(f32, f32)>, fit: FitFunc) -> (f32, f32) {
        match self {
            Size::Fit => fit(items),
            Size::Fill(min, max) => (*min, *max),
            Size::Static(s) => (*s, *s),
            Size::Custom(f) => f(items)
        }
    }

    pub fn max(items: Vec<(f32, f32)>) -> (f32, f32) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }

    pub fn add(items: Vec<(f32, f32)>) -> (f32, f32) {
        items.into_iter().reduce(|s, i| (s.0+i.0, s.1+i.1)).unwrap_or_default()
    }
}

impl std::fmt::Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Fit => write!(f, "Size::Fit"),
            Size::Fill(min, max) => write!(f, "Size::Fill(min: {}, max: {})", min, max),
            Size::Static(val) => write!(f, "Size::Static({})", val),
            Size::Custom(_) => write!(f, "Size::Custom(<function>)"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Padding(pub f32, pub f32, pub f32, pub f32);

impl Padding {
    pub fn new(p: f32) -> Self {Padding(p, p, p, p)}

    pub fn adjust_size(&self, size: (f32, f32)) -> (f32, f32) {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        (size.0-wp, size.1-hp)
    }

    pub fn adjust_offset(&self, offset: (f32, f32)) -> (f32, f32) {
        (offset.0+self.0, offset.1+self.1)
    }

    pub fn adjust_request(&self, request: SizeRequest) -> SizeRequest {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        request.add(wp, hp)
    }
}

pub struct UniformExpand;

impl UniformExpand {
    pub fn get(sizes: Vec<(f32, f32)>, max_size: f32, spacing: f32) -> Vec<f32> {
        // Calculate the total spacing and the minimum size required
        let spacing = (sizes.len() - 1) as f32 * spacing;
        let min_size = sizes.iter().fold(0.0, |s, i| s + i.0) + spacing;

        let mut sizes = sizes.into_iter().map(|s| (s.0, s.1)).collect::<Vec<_>>();

        let mut free_space = (max_size - min_size).max(0.0);
        while free_space > 0.0 {
            let (min_exp, count, next) = sizes.iter().fold((None, 0.0, free_space), |(mut me, mut c, mut ne), size| {
                let min = size.0;
                let max = size.1;
                if min < max { // Item can expand
                    match me {
                        Some(w) if w < min => {
                            ne = ne.min(min - w); // Next size could be the min size of the next expandable block
                        },
                        Some(w) if w == min => {
                            ne = ne.min(max - min); // Next size could be the max size of one of the smallest items
                            c += 1.0;
                        },
                        Some(w) if w > min => {
                            ne = ne.min(max - min).min(w - min); // Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        },
                        _ => {
                            ne = ne.min(max - min); // Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        }
                    }
                }
                (me, c, ne)
            });

            if min_exp.is_none() { break; }
            let min_exp = min_exp.unwrap();

            let expand = (next * count).min(free_space); // Next size could be the rest of the free space
            free_space -= expand;
            let expand = expand / count;

            sizes.iter_mut().for_each(|size| {
                if size.0 < size.1 && size.0 == min_exp {
                    size.0 += expand;
                }
            });
        }

        sizes.into_iter().map(|s| s.0).collect()
    }
}

#[derive(Debug)]
pub struct Row(f32, Offset, Size, Padding);

impl Row {
    pub fn new(spacing: f32, offset: Offset, size: Size, padding: Padding) -> Self {
        Row(spacing, offset, size, padding)
    }

    pub fn center(spacing: f32) -> Self {
        Row::new(spacing, Offset::Center, Size::Fit, Padding::default())
    }
}


impl Layout for Row {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(widths.len()-1) as f32;
        let width = Size::add(widths);
        let height = self.2.get(heights, Size::max);
        self.3.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1).add_width(spacing))
    }

    fn build(&self, _ctx: &mut Context, row_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let row_size = self.3.adjust_size(row_size);

        let widths = UniformExpand::get(children.iter().map(|i| (i.min_width(), i.max_width())).collect::<Vec<_>>(), row_size.0, self.0);

        let mut offset = 0.0;
        children.into_iter().zip(widths).map(|(i, width)| {
            let size = i.get((width, row_size.1));
            let off = self.3.adjust_offset((offset, self.1.get(row_size.1, size.1)));
            offset += size.0+self.0;
            Area{offset: off, size}
        }).collect()
    }
}

#[derive(Debug)]
pub struct Column(f32, Offset, Size, Padding);

impl Column {
    pub fn new(spacing: f32, offset: Offset, size: Size, padding: Padding) -> Self {
        Column(spacing, offset, size, padding)
    }

    pub fn center(spacing: f32) -> Self {
        Column(spacing, Offset::Center, Size::Fit, Padding::default())
    }

    pub fn padding(&mut self) -> &mut Padding {&mut self.3}
}

impl Layout for Column {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(heights.len()-1) as f32;
        let width = self.2.get(widths, Size::max);
        let height = Size::add(heights);
        self.3.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1).add_height(spacing))
    }

    fn build(&self, _ctx: &mut Context, col_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let col_size = self.3.adjust_size(col_size);

        let heights = UniformExpand::get(children.iter().map(|i| (i.min_height(), i.max_height())).collect::<Vec<_>>(), col_size.1, self.0);

        let mut offset = 0.0;
        children.into_iter().zip(heights).map(|(i, height)| {
            let size = i.get((col_size.0, height));
            let off = self.3.adjust_offset((self.1.get(col_size.0, size.0), offset));
            offset += size.1+self.0;
            Area{offset: off, size}
        }).collect()
    }
}

#[derive(Debug, Default)]
pub struct Stack(pub Offset, pub Offset, pub Size, pub Size, pub Padding);

impl Stack {
    pub fn center() -> Self {
        Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default())
    }

    pub fn fill() -> Self {
        Stack(Offset::Center, Offset::Center, Size::fill(), Size::fill(), Padding::default())
    }
}

impl Layout for Stack {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|r|
            ((r.min_width(), r.max_width()), (r.min_height(), r.max_height()))
        ).unzip();
        let width = self.2.get(widths, Size::max);
        let height = self.3.get(heights, Size::max);
        self.4.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1))
    }

    fn build(&self, _ctx: &mut Context, stack_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let stack_size = self.4.adjust_size(stack_size);
        children.into_iter().map(|i| {
            let size = i.get(stack_size);
            let offset = (self.0.get(stack_size.0, size.0), self.1.get(stack_size.1, size.1));
            Area{offset: self.4.adjust_offset(offset), size}
        }).collect()
    }
}

#[derive(Debug)]
pub struct Wrap(pub f32, pub f32, pub Offset, pub Offset, pub Padding, Arc<Mutex<f32>>);

impl Wrap {
    pub fn new(w_spacing: f32, h_spacing: f32) -> Self {
        Wrap(w_spacing, h_spacing, Offset::Start, Offset::Center, Padding::default(), Arc::new(Mutex::new(0.0)))
    }
}

impl Layout for Wrap {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let mut lw = self.4.1;
        let mut lh = 0.0;
        let mut th = self.4.0;
        let mut max_lw: f32 = 0.0;
        for child in children {
            let (w, h) = (child.min_width(), child.min_height());
            if lw + w > *self.5.lock().unwrap() && lw > self.4.1 {
                th += lh + self.1;
                max_lw = max_lw.max(lw - self.0);
                lw = self.4.1;
                lh = 0.0;
            }
            lw += w + self.0;
            lh = lh.max(h);
        }
        if lw > self.4.1 {
            th += lh;
            max_lw = max_lw.max(lw - self.0);
        }
        SizeRequest::new(max_lw + self.4.2, th + self.4.3, f32::MAX, f32::MAX)
    }

    fn build(&self, _ctx: &mut Context, maximum_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        *self.5.lock().unwrap() = maximum_size.0;
        let mut tw = self.4.1;
        let mut ho = self.4.0;
        let mut lh = 0.0;
        let mut areas: Vec<Area> = Vec::new();

        children.into_iter().for_each(|child| {
            let (w, h) = (child.min_width(), child.min_height());
            if (tw + w) > maximum_size.0 && tw > self.4.1 {
                ho += lh + self.1;
                tw = self.4.1;
                lh = 0.0;
            }
            areas.push(Area {offset: (tw, ho), size: (w, h)});
            tw += w + self.0;
            lh = lh.max(h);
        });
        areas
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollAnchor {
    Start,
    End,
}

#[derive(Debug)]
pub struct Scroll(Offset, Offset, Size, Size, Padding, Arc<Mutex<f32>>, ScrollAnchor);

impl Default for Scroll {
    fn default() -> Self {
        Scroll::new(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default(), ScrollAnchor::Start)
    }
}

impl Scroll {
    pub fn new(offset_x: Offset, offset_y: Offset, size_x: Size, size_y: Size, padding: Padding, anchor: ScrollAnchor) -> Self {
        Scroll(offset_x, offset_y, size_x, size_y, padding, Arc::new(Mutex::new(0.0)), anchor)
    }

    pub fn adjust_scroll(&mut self, val: f32) { 
        match self.6 {
            ScrollAnchor::Start => *self.5.lock().unwrap() += val,
            ScrollAnchor::End => *self.5.lock().unwrap() -= val,
        }
    }

    pub fn set_scroll(&mut self, val: f32) { *self.5.lock().unwrap() = val; }
    pub fn offset(&mut self) -> &mut Offset { &mut self.1 }
}

impl Layout for Scroll {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|r|
            ((r.min_width(), r.max_width()), (r.min_height(), r.max_height()))
        ).unzip();
        let width = self.2.get(widths, Size::max);
        let height = self.3.get(heights, Size::max);
        self.4.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1))
    }

    // fn build(&self, _ctx: &mut Context, scroll_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
    //     let scroll_size = self.4.adjust_size(scroll_size);
    //     let children_height: f32 = children.iter().map(|i| i.min_height()).sum();
    //     let max_scroll = (children_height - scroll_size.1).max(0.0);
    //     let scroll_val = self.5.lock().unwrap().clamp(0.0, max_scroll);

    //     *self.5.lock().unwrap() = scroll_val;

    //     children.into_iter().map(|i| {
    //         let size = i.get(scroll_size);
    //         let offset = (self.0.get(scroll_size.0, size.0), self.1.get(scroll_size.1, size.1)-scroll_val);
    //         Area{offset: self.4.adjust_offset(offset), size}
    //     }).collect()
    // }

    fn build(&self, _ctx: &mut Context, scroll_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let scroll_size = self.4.adjust_size(scroll_size);
        let children_height: f32 = children.iter().map(|i| i.min_height()).sum();
        let max_scroll = (children_height - scroll_size.1).max(0.0);

        let mut scroll_val = self.5.lock().unwrap();
        *scroll_val = scroll_val.clamp(0.0, max_scroll);

        children.into_iter().map(|i| {
            let size = i.get(scroll_size);
            let y_offset = match self.6 {
                ScrollAnchor::Start => self.1.get(scroll_size.1, size.1)-*scroll_val,
                ScrollAnchor::End => scroll_size.1 - children_height + *scroll_val,
            };
            let offset = (self.0.get(scroll_size.0, size.0), y_offset);
            Area {offset: self.4.adjust_offset(offset), size }
        }).collect()
    }
}

#[derive(Debug, Component)]
pub struct Bin<L: Layout, D: Drawable>(pub L, pub D);

impl<L: Layout, D: Drawable> OnEvent for Bin<L, D> {}

impl<L: Layout, D: Drawable> Bin<L, D> {
    pub fn inner(&mut self) -> &mut D {
        &mut self.1
    }
    pub fn layout(&mut self) -> &mut L {
        &mut self.0
    }
}

#[derive(Debug, Component)]
pub struct Opt<D: Drawable + 'static>(Stack, Option<D>, #[skip] Option<D>);

impl<D: Drawable + 'static> OnEvent for Opt<D> {}

impl<D: Drawable + 'static> Opt<D> {
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

    pub fn inner(&mut self) -> &mut D {
        self.1.as_mut().unwrap_or_else(|| self.2.as_mut().unwrap())
    }

    pub fn is_showing(&self) -> bool {
        self.1.is_some()
    }
}

#[derive(Debug, Component)]
pub struct EitherOr<L: Drawable + 'static, R: Drawable + 'static>(Stack, Opt<L>, Opt<R>);

impl<L: Drawable + 'static, R: Drawable + 'static> OnEvent for EitherOr<L, R> {}

impl<L: Drawable + 'static, R: Drawable + 'static> EitherOr<L, R> {
    pub fn new(left: L, right: R) -> Self {
        EitherOr(Stack::default(), Opt::new(left, true), Opt::new(right, false))
    }

    pub fn display_left(&mut self, display_left: bool) {
        self.1.display(display_left);
        self.2.display(!display_left);
    }

    pub fn left(&mut self) -> &mut L { self.1.inner() }
    pub fn right(&mut self) -> &mut R { self.2.inner() }
}
