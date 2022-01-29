
use egui::Color32;
use crate::prelude::*;

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

#[derive(Debug, Clone)]
pub struct FloatRange {
    from: f32,
    to: f32
}

impl FloatRange {
    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<FloatRange, AbstractSyntaxTokenError> {
        match collect_array_floats(value, 2) {
            Ok(values) => Ok(FloatRange { from: values[0], to: values[1] }),
            Err(_) => Err(AbstractSyntaxTokenError::RangeValueParseError)
        }
    }
}

impl From<&FloatRange> for RangeInclusive<f32> {
    fn from(from: &FloatRange) -> Self {
        Self::new(from.from, from.to)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedIntRange {
    from: u32,
    to: u32
}

impl UnsignedIntRange {
    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<UnsignedIntRange, AbstractSyntaxTokenError> {
        match collect_array_unsigned_ints(value, 2) {
            Ok(values) => Ok(UnsignedIntRange { from: values[0], to: values[1] }),
            Err(_) => Err(AbstractSyntaxTokenError::RangeValueParseError)
        }
    }
}

impl From<&UnsignedIntRange> for RangeInclusive<u32> {
    fn from(from: &UnsignedIntRange) -> Self {
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
    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<Colour, AbstractSyntaxTokenError> {
        match collect_array_unsigned_shorts(value, 4) {
            Ok(values) => Ok(
                Colour { 
                    r: values[0] as u8,
                    g: values[1] as u8,
                    b: values[2] as u8,
                    a: values[3] as u8 
                }
            ),
            Err(_err) => Err(AbstractSyntaxTokenError::ColourValueParseError)
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

