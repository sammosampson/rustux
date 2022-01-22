
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
    Sidebar,
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
    HorizontalOrientation(HorizontalOrientation),
    VerticallySized(VerticalSize)
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalOrientation {
    Left,
    Right
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