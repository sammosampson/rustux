
mod stream;
mod building;

pub use stream::*;
pub use building::*;

use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnknownProperty(String),
    UnknownPropertyValue(String),
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
    DefaultWidth(f32),
    DefaultHeight(f32),
    WidthRange(FloatRange),
    HeightRange(FloatRange),
    VerticallySized(VerticalSize)
}

#[derive(Debug, Clone)]
pub struct FloatRange(RangeInclusive<f32>);

impl Deref for FloatRange {
    type Target = RangeInclusive<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FloatRange {
    pub fn parse(value: &str) -> Result<FloatRange, SpecificCollectionError> {
        match collect_tuple_floats(value, 2) {
            Ok(values) => Ok(FloatRange(RangeInclusive::new(values[0], values[1]))),
            Err(err) => Err(err)
        }
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