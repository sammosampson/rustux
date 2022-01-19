
mod stream;
mod building;

pub use stream::*;
pub use building::*;

use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnusedPropertyType,
    UnknownProperty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractSyntaxTokenType {
    Unknown,
    Root,
    LeftSidebar,
    RightSidebar,
}

impl Default for AbstractSyntaxTokenType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxToken {
    Start(AbstractSyntaxTokenType),
    End(AbstractSyntaxTokenType),
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;