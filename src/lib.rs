#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/logo.png")]

//! Pelican UI Standard provides a wide range of components and interface systems for building beautiful, consistently designed applications.
//!
//! Download the starter template here: <https://github.com/EllaCouch20/ramp_template>

//! ## Components
//!
//! Components are the main purpose of Pelican UI Standard 
//! and contains commonly used UI and interface components 
//! composed from elements and layouts. 
//!
//! Every structure implementing the [`Component`](pelican_ui::Component) trait must have its **first element implement [`Layout`](pelican_ui::layout::Layout)**.  
//! This ensures that the component can correctly manage positioning, sizing, and nested layouts within the UI hierarchy.
//!
//! Every structure implementing the [`Component`](pelican_ui::Component) trait must also implement [`OnEvent`](pelican_ui::events::OnEvent).
//!
//! Every structure implementing the [`Component`](pelican_ui::Component) trait must also derive [`Debug`].
//!
//! ## Example
//!
//! ```rust
//! #[derive(Debug, Component)]
//! pub struct InfoCard(Column, Text, Text);
//! impl OnEvent for InfoCard {}

//! impl InfoCard {
//!     pub fn new(ctx: &mut Context, title: &str, description: &str) -> Self {
//!         let font_size = ctx.theme.fonts.size;

//!         InfoCard(
//!             Column::new(8.0, Offset::Center, Size::Fit, Padding::new(16.0)),
//!             Text::new(ctx, title, TextStyle::Header, font_size.h3, Align::Left),
//!             Text::new(ctx, description, TextStyle::Primary, font_size.md, Align::Left)
//!         )
//!     }
//! }
//! ```
//!
//! ## Events
//!
//! Pelican UI Standard contains multiple [`Event`](pelican_ui::events::Event)s that
//! are used/triggered by Pelican UI Standard components.
//!
//! ## Config & Utils
//!
//! Pelican UI Standard contains configuration variables for detecting if the user is on mobile ([`IS_MOBILE`]) or web ([`IS_WEB`]).
//!
//! Pelican UI Standard contains utility structures like [`Timestamp`] and [`ElementID`]
//!
//! ## Pages
//!
//! Pelican UI Standard contains a couple pages built from its own components. 
//!
//! - [`PelicanHome`]
//! - [`Error`]
//! - [`Splash`]
//!
//! These can be used as is or as examples when building new pages.
//!
//! ## Layout
//!
//! Every structure implementing the [`Component`](pelican_ui::Component) trait must have its **first element implement [`Layout`](pelican_ui::layout::Layout)**.  
//! This ensures that the component can correctly manage positioning, sizing, and nested layouts within the UI hierarchy.
//!
//! Pelican UI Standard contains multiple layouts such as [`Column`], [`Row`], [`Stack`] and more.
//!
//! ## Elements
//!
//! Elements are the lowest-level UI primitives, such as [`Text`], [`AspectRatioImage`], [`Circle`], and other  
//! fundamental building blocks that can be composed into larger components.
//!

mod events;
pub use events::{
    NavigateEvent,
    KeyboardActiveEvent,
    ClearActiveInput,
    SetActiveInput,
    TextInputSelect,
    ListItemSelect,
    NavigatorSelect,
    NavigatorEvent,
    SearchEvent,
    InputEditedEvent,
    AdjustScrollEvent,
    QRCodeScannedEvent,
    AttachmentEvent,
};

mod config;
pub use config::{IS_MOBILE, IS_WEB};

mod layout;
pub use layout::{
    Offset, 
    Size, 
    Padding, 
    Column, 
    Row, 
    Wrap, 
    Scroll, 
    Stack, 
    ScrollAnchor, 
    Bin, 
    Opt, 
    EitherOr
};

mod elements;
pub use elements::{
    Text, 
    ExpandableText, 
    TextStyle, 
    BulletedText,
    ExpandableImage, 
    EncodedImage,
    AspectRatioImage, 
    Icon, 
    Circle, 
    Rectangle,
    RoundedRectangle, 
    OutlinedRectangle,
    Outline,
};

mod components;
pub use components::{
    Button,
    ButtonStyle,
    ButtonSize,
    ButtonState,
    ButtonWidth,
    IconButton,
    QuickActions,
    Alert,
    Avatar,
    AvatarContent,
    AvatarIconStyle,
    DataItem,
    ListItem,
    ListItemGroup,
    ListItemSelector,
    TextInput,
    Searchbar,
    QRCode,
    QRCodeScanner,
    Slider,
    Interface,
    Page,
    Header,
    Bumper,
    Content,
};

mod utils;
pub use utils::{
    Timestamp, 
    ElementID
};

mod pages;
pub use pages::{
    AppPage, 
    Error, 
    Splash, 
    PelicanHome
};

/*
How to create an app with Ramp
How to create a page with Ramp
How to create a component with Ramp
How to create a layout with Ramp
How to create a plugin with Ramp
How to create a messaging app with Ramp
How to create a bitcoin wallet with Ramp
How to create a AIR profile with Ramp
*/