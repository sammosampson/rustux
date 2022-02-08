use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    CodeTokenError(CodeTokenError),
    UnknownProperty(String),
    UnknownPropertyValue(String),
    RangeValueParseError,
    ColourValueParseError,
    VariablePathParseError(String),
    TextStyleValueParseError(String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractSyntaxControlType {
    Unknown,
    Root,
    Control,
    Import,
    Container,
    For,
    ForEach,
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

impl Default for AbstractSyntaxControlType {
    fn default() -> Self {
        Self::Unknown
    }
}

pub fn create_ast_property(
    property_type: AbstractSyntaxPropertyType,
    value: AbstractSyntaxPropertyValue
) -> AbstractSyntaxProperty {
    AbstractSyntaxProperty(property_type, value)
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxProperty(AbstractSyntaxPropertyType, AbstractSyntaxPropertyValue);

impl AbstractSyntaxProperty {
    pub fn property_type(&self) -> &AbstractSyntaxPropertyType {
        &self.0
    }

    pub fn value(&self) -> &AbstractSyntaxPropertyValue {
        &self.1
    }

    pub fn set_value(&self, value: AbstractSyntaxPropertyValue) -> AbstractSyntaxProperty {
        AbstractSyntaxProperty(self.0.clone(), value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AbstractSyntaxPropertyType {
    Id,
    Name,
    Path,
    Text,
    Selected,
    Resizable,
    Wrap,
    Code,
    Strong,
    Weak,
    Strikethrough,
    Underline,
    Italics,
    Raised,
    TextStyle,
    DefaultWidth,
    DefaultHeight,
    WidthRange,
    HeightRange,
    VerticallySized, 
    AutoSized, 
    AlwaysShowScroll,
    ScrollOffset,
    EnableScrolling,
    Colour, 
    BackgroundColour,
    OnSelect,
    USizeRangeVariable,
    FunctionVariable
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxToken {
    StartControl(AbstractSyntaxControlType),
    Property(AbstractSyntaxProperty),
    EndControl(AbstractSyntaxControlType),
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;