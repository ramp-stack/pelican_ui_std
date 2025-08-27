//! ==! FirstScreen Page Tutorial
//!
//! This example teaches you how to create a full-page component step by step using Pelican UI.
//!
//! ==! 1. Define the Page Struct
//!
//! Start by defining your page struct. It will contain a `Stack` and a `Page`:
//!
//! ```rust
//! #[derive(Debug, Component)]
//! pub struct FirstScreen(Stack, Page);
//! ```
//!
//! ==! 2. Implement `OnEvent`
//!
//! This allows the page to receive events (like clicks or input):
//!
//! ```rust
//! impl OnEvent for FirstScreen {}
//! ```
//!
//! ==! 3. Implement `AppPage`
//!
//! Implement `AppPage` to integrate with your app's navigation system:
//!
//! ```rust
//! impl AppPage for FirstScreen {
//!     fn has_nav(&self) -> bool { false } // no navigation bar
//!
//!     fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) 
//!         -> Result<Box<dyn AppPage>, Box<dyn AppPage>> 
//!     { 
//!         Err(self) // navigation not implemented
//!     }
//! }
//! ```
//!
//! ==! 4. Create the Constructor
//!
//! Define a `new` function to build the page content:
//!
//! ```rust
//! impl FirstScreen {
//!     pub fn new(ctx: &mut Context) -> Self {
//!         // 4a. Choose a heading color
//!         let color = ctx.theme.colors.text.heading;
//!
//!         // 4b. Create an icon
//!         let icon = Icon::new(ctx, "pelican_ui", color, 128.0);
//!
//!         // 4c. Create heading and subtext
//!         let font_size = ctx.theme.fonts.size;
//!         let text = Text::new(ctx, "Hello World!", TextStyle::Heading, font_size.h2, Align::Center);
//!         let subtext = ExpandableText::new(ctx, "First project loaded successfully.", TextStyle::Primary, font_size.md, Align::Center, None);
//!
//!         // 4d. Combine content into a single column
//!         let content = Content::new(ctx, Offset::Center, vec![Box::new(icon), Box::new(text), Box::new(subtext)]);
//!
//!         // 4e. Create a header
//!         let header = Header::home(ctx, "My Screen", None);
//!
//!         // 4f. Construct the page
//!         FirstScreen(Stack::default(), Page::new(Some(header), content, None))
//!     }
//! }
//! ```
//!
//! ==! Summary
//!
//! 1. Define a struct holding `Stack` and `Page`.  
//! 2. Implement `OnEvent` for interactivity.  
//! 3. Implement `AppPage` for navigation integration.  
//! 4. Build the constructor step by step:
//!    - Choose colors  
//!    - Create icon, heading, and subtext  
//!    - Combine them in `Content`  
//!    - Add a `Header`  
//!    - Return the `FirstScreen` instance
