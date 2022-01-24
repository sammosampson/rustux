
mod stream;
mod building;

use egui::Color32;
pub use stream::*;
pub use building::*;

use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnknownProperty(String),
    UnknownPropertyValue(String),
    RangeValueParseError(String),
    ColourValueParseError(String),
    TextStyleValueParseError(String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractSyntaxTokenType {
    Unknown,
    Root,
    CentralPanel,
    TopPanel,
    BottomPanel,
    LeftSidebar,
    RightSidebar,
    Horizontal,
    Vertical,
    ScrollArea,
    Separator,
    Label,
    ColouredLabel,
    SelectableLabel,
    Heading,
    Monospace,
    Code
}

impl Default for AbstractSyntaxTokenType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxTokenProperty {
    Id(String),
    Text(String),
    Selected(bool),
    Resizable(bool),
    Wrap(bool),
    Code(bool),
    Strong(bool),
    Weak(bool),
    Strikethrough(bool),
    Underline(bool),
    Italics(bool),
    Raised(bool),
    TextStyle(TextStyle),
    DefaultWidth(f32),
    DefaultHeight(f32),
    WidthRange(FloatRange),
    HeightRange(FloatRange),
    VerticallySized(VerticalSize), 
    Colour(Colour), 
    BackgroundColour(Colour)
}

#[derive(Debug, Clone)]
pub struct FloatRange {
    from: f32,
    to: f32
}

#[derive(Debug, Clone)]
pub enum TextStyle {
    Small,
    Body,
    Button,
    Heading,
    Monospace,
}

impl TextStyle {
    pub fn parse(value: &str) -> Result<TextStyle, AbstractSyntaxTokenError> {
        match value {
            "small" => Ok(TextStyle::Small),
            "body" => Ok(TextStyle::Body),
            "button" => Ok(TextStyle::Button),
            "heading" => Ok(TextStyle::Heading),
            "monospace" => Ok(TextStyle::Monospace),
            _ => Err(AbstractSyntaxTokenError::TextStyleValueParseError(value.to_string()))
        }
    }
}

impl From<&TextStyle> for egui::TextStyle {
    fn from(from: &TextStyle) -> Self {
        match from {
            TextStyle::Small => egui::TextStyle::Small,
            TextStyle::Body => egui::TextStyle::Body,
            TextStyle::Button => egui::TextStyle::Button,
            TextStyle::Heading => egui::TextStyle::Heading,
            TextStyle::Monospace => egui::TextStyle::Monospace,
        }
    }
}

impl FloatRange {
    pub fn parse(value: &str) -> Result<FloatRange, AbstractSyntaxTokenError> {
        match collect_tuple_floats(value, 2) {
            Ok(values) => Ok(FloatRange { from: values[0], to: values[1] }),
            Err(_err) => Err(AbstractSyntaxTokenError::RangeValueParseError(value.to_string()))
        }
    }
}

impl From<&FloatRange> for RangeInclusive<f32> {
    fn from(from: &FloatRange) -> Self {
        Self::new(from.from, from.to)
    }
}

#[derive(Debug, Clone)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Colour {
    pub fn parse(value: &str) -> Result<Colour, AbstractSyntaxTokenError> {
        match collect_tuple_unsigned_shorts(value, 4) {
            Ok(values) => Ok(
                Colour { 
                    r: values[0] as u8,
                    g: values[1] as u8,
                    b: values[2] as u8,
                    a: values[3] as u8 
                }
            ),
            Err(_err) => Err(AbstractSyntaxTokenError::ColourValueParseError(value.to_string()))
        }
    }
}

impl From<&Colour> for Color32 {
    fn from(from: &Colour) -> Self {
        Self::from_rgba_unmultiplied(from.r, from.g, from.b, from.a)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalSize {
    Auto,
    MaxHeight(f32)
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxToken {
    StartNode(AbstractSyntaxTokenType),
    Property(AbstractSyntaxTokenProperty),
    EndNode(AbstractSyntaxTokenType),
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;