use rust_on_rails::prelude::*;
use resources::Font;
//use pelican_ui::prelude::*;

use std::sync::Arc;

//  #[derive(Clone)]
//  pub struct Column(pub u32, pub Vec<BoxComponent>);

//  impl LayoutComponent for Column {
//      fn children(&self) -> Vec<&BoxComponent> {self.1.iter().collect()}
//      fn children_mut(&self) -> Vec<&mut BoxComponent> {self.1.iter_mut().collect()}

//      fn layout(&self, ctx: &mut , max_size: (u32, u32), items: Vec<fn (u32, u32) -> (u32, u32)>) -> Vec<((i32, i32), (u32, u32))> {
//          let mut offset = 0;
//          items.iter_mut().flat_map(|c| {
//              if max_size.1 < offset {None} else {
//                  //let size = c.size(ctx, (max_size.0, max_size.1-offset));
//                  let size = todo!();
//                  let v = ((0, offset as i32), size);
//                  offset += size.1+self.0;
//                  v
//              }
//          }).collect()

//      }
//  }

#[derive(Clone, Copy, Default, Debug)]
pub enum Size {
    #[default]
    Fit,
    Expand,
    Fill,
    Static(u32, u32),
    Custom(fn((u32, u32), (u32, u32)) -> (u32, u32))
}

impl Size {
    pub fn get(&self, max_size: (u32, u32), c_size: (u32, u32)) -> (u32, u32) {
        match self {
            Self::Fit => (c_size.0.min(max_size.0), c_size.1.min(max_size.1)),
            Self::Expand => c_size,
            Self::Fill => max_size,
            Self::Static(x, y) => (*x, *y),
            Self::Custom(func) => func(max_size, c_size)
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
    Static(i32, i32),
    #[allow(clippy::type_complexity)]
    Custom(fn((u32, u32), (u32, u32)) -> (i32, i32))
}

impl Offset {
    pub fn get(&self, max_size: (u32, u32), min_size: (u32, u32)) -> (i32, i32) {
        match self {
            Self::TopLeft => (0, 0),
            Self::TopCenter => (((max_size.0 - min_size.0) / 2) as i32, 0),
            Self::TopRight => ((max_size.0 - min_size.0) as i32, 0),
            Self::Left => (0, ((max_size.1 - min_size.1) / 2) as i32),
            Self::Center => (((max_size.0 - min_size.0) / 2) as i32, ((max_size.1 - min_size.1) / 2) as i32),
            Self::Right => ((max_size.0 - min_size.0) as i32, ((max_size.1 - min_size.1) / 2) as i32),
            Self::BottomLeft => (0, (max_size.1 - min_size.1) as i32),
            Self::BottomCenter => (((max_size.0 as i32 - min_size.0 as i32) / 2), (max_size.1 as i32 - min_size.1 as i32)),
            Self::BottomRight => ((max_size.0 as i32 - min_size.0 as i32), (max_size.1 as i32 - min_size.1 as i32)),
            Self::Static(x, y) => (*x, *y),
            Self::Custom(func) => func(max_size, min_size)
        }
    }
}


#[derive(Clone, Debug)]
pub struct Stack;//Offset::TopLeft, Size::Fit

impl Layout for Stack {
    fn layout(&self, ctx: &mut Context, max_size: (u32, u32), items: Vec<SizeFn>) -> Vec<((i32, i32), (u32, u32))> {
        items.into_iter().map(|mut i| ((0, 0), i(ctx, max_size))).collect()
    }
}


#[derive(Clone, Debug)]
pub struct Column(pub u32);

impl Layout for Column {
    fn layout(&self, ctx: &mut Context, max_size: (u32, u32), items: Vec<SizeFn>) -> Vec<((i32, i32), (u32, u32))> {
        let mut offset = 0;
        items.into_iter().map(|mut i| {
            //let size = c.size(ctx, (max_size.0, max_size.1-offset));
            let size = i(ctx, (max_size.0, max_size.1-offset));
            let v = ((0, offset as i32), size);
            offset += size.1+self.0;
            v
        }).collect()
    }
}

#[derive(Clone, Debug)]
pub struct Test(Font, Text, Text);

impl Test {
    pub fn new(font: Font, font2: Font) -> Self {
        Test(font2.clone(),
            Text::new("HELLO HE", Color::from_hex("eb343a", 255), 48, 60, font.clone()),
            Text::new("HELLO HL", Color::from_hex("eb343a", 255), 24, 30, font.clone()),
        )
    }
}

impl Component for Test {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container::new(Column(50), vec![&mut self.1, &mut self.2])
        //Container::new(Stack::default(), vec![&mut self.1, &mut self.2])
        container![&mut self.1, &mut self.2]
    }
}


  //fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<((i32, i32), (u32, u32), &mut BoxComponent)> {
  //    let mut offset = 0;
  //    self.1.iter_mut().flat_map(|c| {
  //        if max_size.1 < offset {None} else {
  //            let size = c.size(ctx, (max_size.0, max_size.1-offset));
  //            let v = ((0, offset as i32), size, c);
  //            offset += size.1+self.0;
  //            Some(v)
  //        }
  //    }).collect()
  //}
    //fn children_mut(&mut self) -> Vec<&mut BoxComponent> {self.1.iter_mut().collect()}

  //fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
  //    let mut offset = 0;
  //    for c in &mut self.1 {
  //        let size = c.size(ctx, (max_size.0, max_size.1-offset));
  //        if size.0 > position.0 && offset+size.1 > position.1 {
  //            c.on_click(
  //                ctx,
  //                (max_size.0, offset+self.0),
  //                (position.0, position.1-offset)
  //            );
  //            return;
  //        }
  //        if offset+size.1+self.0 > position.1 {
  //            return;
  //        }
  //        offset += self.0 + size.1;
  //    }
  //}

//  #[derive(Clone)]
//  pub struct MyCom{
//      font: resources::Font,
//      text: String
//  }

//  impl Component for MyCom {
//      fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<(Offset, Size, BoxComponent)> {
//          vec![
//              (Offset::Center, Size::Static(900, 200), vec![Box::new(
//                  Shape(ShapeType::Rectangle(0), Color(0, 0, 255, 255)),
//                  Image(ShapeType::Rectangle(0), image, Some(Color(255, 0, 0, 128))
//              ]) as BoxComponent),




//              (Offset::Center, Size::default(), Box::new(
//                  Text(self.text.clone(), Color::from_hex("eb343a", 255), 48, 60, self.font.clone())
//              )),
//              (Offset::BottomRight, Size::default(), Box::new(
//                  Text(self.text.clone(), Color::from_hex("eb343a", 255), 48, 60, self.font.clone())
//              ))
//          ]

//        //container!(Offset::BottomRight, Size::Expand, [
//        //    Shape(ShapeType::Rectangle(0), Color(0, 0, 255, 255)).size(Size::Static(1000, 200))
//        //    text
//        //])

//        //vec![Box::new(Container(Offset::BottomCenter, Size::Static(500, 200),
//        //    vec![
//        //        Box::new(Shape(ShapeType::Rectangle(0), Color(0, 0, 255, 255))) as Box<dyn Component>,
//        //        Box::new(text) as Box<dyn Component>,
//        //    ]
//        //))]
//      }

//      fn on_click(&mut self, ctx: &mut Context, max_size: (u32, u32), position: (u32, u32)) {
//          self.text = format!("{:?}", position);
//      }
//  }

pub struct MyApp;

impl App for MyApp {
    async fn new(ctx: &mut Context<'_>) -> BoxComponent {
        //PelicanContext.init(ctx);
        println!("START APP");

        let font = resources::Font::new(ctx, include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());
        let font2 = resources::Font::new(ctx, include_bytes!("../assets/fonts/outfit_regular.ttf").to_vec());
        Box::new(Test::new(font, font2))
      //let image = resources::Image::new(ctx, image::load_from_memory(include_bytes!("../assets/icons/pfp3.png")).unwrap().into());
      //let svg = resources::Image::svg(ctx, include_bytes!("../assets/icons/qr_code.svg"), 8.0);



      //Box::new(Container(Column(50), vec![
      //    //Some(Text("HELLO WORLD HEL", Color::from_hex("eb343a", 255), 48, 60, font.clone())).into(),
      //    //Box::new(None::<Text>)
      ////Box::new(Column(0, vec![

      //    //Box::new(MyCom{font: font.clone(), text: "HEL".to_string()})
      //    Box::new(Text::new("HELLO WORLD HEL", Color::from_hex("eb343a", 255), 48, 60, font.clone())),
      //    Box::new(Text::new("HELLO HE", Color::from_hex("eb343a", 255), 48, 60, font.clone())),
      //    Box::new(Text::new("HELLO HL", Color::from_hex("eb343a", 255), 24, 30, font.clone())),
      //    Box::new(Text::new("HELLO EL", Color::from_hex("eb343a", 255), 48, 60, font.clone())),
      //]))
      //]))
        //Box::new(Icon::new(ctx, IconName::Door, "ffffff", 48))
    }
}

create_entry_points!(MyApp);
