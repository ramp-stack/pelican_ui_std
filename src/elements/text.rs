use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::layout::{Stack, Offset, Size, Padding, Opt, Row};
use crate::elements::shapes::{Rectangle, Circle};
use crate::PelicanUI;

/// Enumeration of text styles used in the UI.
#[derive(Clone, Copy, Debug)]
pub enum TextStyle {
    /// Represents the heading or title text style. Used for prominent titles or section headers.
    Heading,
    /// Represents the primary body text style. Typically used for main content text.
    Primary,
    /// Represents the secondary text style. Used for secondary or less prominent content.
    Secondary,
    /// Represents the error text style. Often used to indicate error messages or alerts.
    Error,
    /// Represents the white text style. Used when white-colored text is needed.
    White,
    /// Represents the keyboard text style. Used for the keyboard key/label text.
    Keyboard,
    /// Represents a label text style with a custom color. The `Color` parameter allows customization of the text color.
    Label(Color),
}

impl TextStyle {
    /// Retrieves the color and font associated with the `TextStyle`.
    pub fn get(&self, ctx: &mut Context) -> (Color, resources::Font) {
        let theme = &ctx.get::<PelicanUI>().theme;
        match self {
            TextStyle::Heading => (theme.colors.text.heading, theme.fonts.fonts.heading.clone()),
            TextStyle::Primary => (theme.colors.text.primary, theme.fonts.fonts.text.clone()),
            TextStyle::Secondary => (theme.colors.text.secondary, theme.fonts.fonts.text.clone()),
            TextStyle::Error => (theme.colors.status.danger, theme.fonts.fonts.text.clone()),
            TextStyle::White => (theme.colors.text.heading, theme.fonts.fonts.text.clone()),
            TextStyle::Keyboard => (theme.colors.text.heading, theme.fonts.fonts.keyboard.clone()),
            TextStyle::Label(color) => (*color, theme.fonts.fonts.label.clone()),
        }
    }
}

/// Component representing a text cursor.
#[derive(Component, Debug)]
pub struct TextCursor(Stack, Opt<Rectangle>);

impl OnEvent for TextCursor {}

impl TextCursor {
    /// Creates a new `TextCursor` with the specified style and size.
    pub fn new(ctx: &mut Context, style: TextStyle, size: f32) -> Self {
        let (color, _) = style.get(ctx);
        TextCursor(
            Stack(Offset::Start, Offset::End, Size::Static(2.0), Size::Static(size), Padding::default()), 
            Opt::new(Rectangle::new(color), false)
        )
    }

    /// Displays or hides the cursor.
    pub fn display(&mut self, display: bool) { self.1.display(display) }

    /// Returns the X offset of the cursor.
    pub fn x_offset(&mut self) -> &mut Offset { &mut self.0.0 }

    /// Returns the Y offset of the cursor.
    pub fn y_offset(&mut self) -> &mut Offset { &mut self.0.1 }
}

/// Component representing a text element, with or without a cursor.
#[derive(Component, Debug)]
pub struct Text(Stack, BasicText, Option<TextCursor>);

impl Text {
    /// Creates a new `Text` component with the given text, style, size, and alignment.
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text, size, size*1.25, font, color)], None, align, None);
        Text(Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()), text, None)
    }

    /// Creates a new `Text` component with a cursor, along with the given text, style, size, and alignment.
    pub fn new_with_cursor(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text, size, size*1.25, font, color)], None, align, Some(Cursor::default()));
        Text(
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()),
            text, Some(TextCursor::new(ctx, style, size)),
        )
    }

    /// Returns a mutable reference to the `BasicText` of the `Text` component.
    pub fn text(&mut self) -> &mut BasicText { &mut self.1 }

    /// Returns a mutable reference to the `TextCursor` (if any) of the `Text` component.
    pub fn cursor(&mut self) -> &mut Option<TextCursor> { &mut self.2 }
}

impl OnEvent for Text {
    /// Handles events, such as cursor movements or mouse clicks, for the `Text` component.
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(cursor) = &mut self.2 {
            if event.downcast_ref::<TickEvent>().is_some() {
                if let Some(cords) = self.1.cursor_action(ctx.as_canvas(), CursorAction::GetPosition) {
                    *cursor.x_offset() = Offset::Static(cords.0);
                    *cursor.y_offset() = Offset::Static(cords.1-(self.1.spans[0].line_height/1.2));
                }
            } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
                if event.state == MouseState::Pressed && event.position.is_some() {
                    self.1.set_cursor(ctx.as_canvas(), (event.position.unwrap().0, event.position.unwrap().1));
                    self.1.cursor_action(ctx.as_canvas(), CursorAction::GetPosition);
                }
            }
        }
        true
    }
}

/// Component representing a text element that can expand based on its content.
#[derive(Debug)]
pub struct ExpandableText(pub Text);

impl ExpandableText {
    /// Creates a new `ExpandableText` component with the given text, style, size, and alignment.
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        ExpandableText(Text::new(ctx, text, style, size, align))
    }

    /// Creates a new `ExpandableText` component with a cursor, along with the given text, style, size, and alignment.
    pub fn new_with_cursor(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        ExpandableText(Text::new_with_cursor(ctx, text, style, size, align))
    }

    /// Returns a mutable reference to the `BasicText` of the `ExpandableText` component.
    pub fn text(&mut self) -> &mut BasicText { self.0.text() }

    /// Returns a mutable reference to the `TextCursor` (if any) of the `ExpandableText` component.
    pub fn cursor(&mut self) -> &mut Option<TextCursor> { self.0.cursor() }
}

impl OnEvent for ExpandableText {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if event.downcast_ref::<TickEvent>().is_some() && self.0.cursor().is_some() {
                let s = break_all_ligatures(&self.0.text().spans[0].text.clone());
                self.0.text().spans[0].text = s;
        }
        true
    }
}

impl Component for ExpandableText {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.0] }
    fn children(&self) -> Vec<&dyn Drawable> { vec![&self.0] }

    fn request_size(&self, ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        let height = self.0.1.size(ctx).1;
        SizeRequest::new(0.0, height, f32::MAX, height)
    }

    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        self.0.text().width = Some(size.0);
        vec![Area{offset: (0.0, 0.0), size}]
    }
}

/// A component that represents bulleted text, combining a row layout, circular bullet, and expandable text.
///
/// The `BulletedText` component is designed to display a piece of text with a bullet (circle) preceding it.
/// The bullet and text are arranged in a row layout, and the text is expandable. This component supports styling
/// through `TextStyle` and alignment via `Align`.
///
/// # Fields
/// - `Row`: A layout component that arranges the bullet and text in a row, with configurable size, alignment, and padding.
/// - `Shape`: A circular bullet shape that precedes the text. The size of the bullet is proportional to the provided `size`.
/// - `ExpandableText`: The text component that holds the actual text, with support for styling and expandable behavior.
///
/// # Example
/// ```rust
/// let ctx: &mut Context = ...;
/// let bulleted_text = BulletedText::new(ctx, "This is a bulleted item.", TextStyle::default(), 20.0, Align::Left);
/// ```
#[derive(Debug, Component)]
pub struct BulletedText(Row, Shape, ExpandableText);

impl OnEvent for BulletedText {}

impl BulletedText {
    /// Creates a new `BulletedText` component.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `text`: The static text to display next to the bullet.
    /// - `style`: The text style used for styling the text.
    /// - `size`: The size of the bullet and text.
    /// - `align`: The alignment of the text.
    ///
    /// # Returns
    /// A new `BulletedText` component containing a row with a bullet and expandable text.
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, _) = style.get(ctx);
        BulletedText(
            Row::new(size*0.75, Offset::Center, Size::Fit, Padding::default()), // change this offset to be line_height - circle size / 2
            Circle::new(size*0.5, color),
            ExpandableText::new(ctx, text, style, size, align)
        )
    }

    /// Returns a mutable reference to the `BasicText` component inside the `ExpandableText` part of `BulletedText`.
    ///
    /// This method allows direct manipulation of the text within the `ExpandableText` component.
    ///
    /// # Returns
    /// A mutable reference to the `BasicText` component for modifying the text.
    pub fn text(&mut self) -> &mut BasicText { self.2.text() }
}

/// Breaks text ligatures to prevent tt and ff from becoming a single glyph
pub fn break_all_ligatures(s: &str) -> String {
    const ZWNJ: char = '\u{200C}';

    let mut result = String::new();
    let mut chars = s.chars().peekable();

    if let Some(first) = chars.next() {
        result.push(first);

        while let Some(&c) = chars.peek() {
            // println!("NEXT {:?}", c);
            if result.ends_with(ZWNJ) || c == ZWNJ {
                result.push(c);
            } else {
                result.push(ZWNJ);
                result.push(c);
            }
            chars.next();
        }
    }

    result
}
