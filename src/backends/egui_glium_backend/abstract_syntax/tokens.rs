use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    CodeTokenError(CodeTokenError),
    UnknownProperty(String),
    UnknownPropertyValue(String),
    RangeValueParseError,
    ColourValueParseError,
    TextStyleValueParseError(String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractSyntaxTokenType {
    Unknown,
    Root,
    For,
    Let,
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
    AlwaysShowScroll(bool),
    ScrollOffset(f32),
    EnableScrolling(bool),
    Colour(Colour), 
    BackgroundColour(Colour),
    OnSelect(Function),
    UnsignedIntRangeVariable(String, UnsignedIntRange),
    FunctionVariable(String, Function)
}


#[derive(Debug, Clone)]
pub enum AbstractSyntaxToken {
    StartNode(AbstractSyntaxTokenType),
    Property(AbstractSyntaxTokenProperty),
    VariableProperty(String),
    EndNode(AbstractSyntaxTokenType),
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;