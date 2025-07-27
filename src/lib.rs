#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui/main/logo.png")]

pub mod events;
pub mod config;
pub mod layout;
pub mod elements;
pub mod components;
pub mod utils;
pub mod pages;

pub use crate::pages::*;
pub use crate::events::*;
pub use crate::layout::*;
pub use crate::components::*;
pub use crate::elements::*;
pub use crate::utils::*;
pub use crate::config::*;

/*

Pelican UI Standard is a library of components and tools for building applications

ELEMENTS

Pelican UI contains 3 basic elements
1. Images
Pelican UI Standard has 4 Image 'helpers'

 - Icon
    this is used for creating an icon image from the theme icons
 - AspectRatioImage
    display an image retaining the aspect ratio of the image file
 - EncodedImage
    Encode image bytes into a string for storage and later decoding
 - ExpandableImage
    This image will expand to fill available space

2. Shapes

Pelican UI Standard has 5 shapes
    - Rectangle
    - RoundedRectangle
    - OutlinedRectangle (Rounded)
    - Outline (Circle)
    - Circle
 
3. Text

 - Text
 - ExpandableText
 - TextEditor
 - TextCursor
 - BulletedText
 - TextStyle
*/

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