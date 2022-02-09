mod source;
mod collections;
mod collection;
mod code;
mod lookup;
mod tests;

pub use source::*;
pub use collections::*;
pub use collection::*;
pub use code::*;
pub use lookup::*;

pub const FUNCTION_OPENING_BRACE: char = '(';
pub const FUNCTION_CLOSING_BRACE: char = ')';
pub const ARRAY_OPENING_CHAR: char = '[';
pub const ARRAY_CLOSING_CHAR: char = ']';
pub const CODE_OPENING_CHAR: char = '{';
pub const CODE_CLOSING_CHAR: char = '}';

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SourceTokenError {
    CouldNotFindStartTag(usize),
    CouldNotParseNumberValue(usize),
    CouldNotFindControlName(usize),
    CouldNotFindPropertyStartSymbol(usize),
    CouldNotFindControlToClose(usize),
    CouldNotFindControlCloseSymbol(usize),
    ClosingWrongTag(usize)
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum SourceTokenPropertyValue {
    String(String),
    Int(i128),
    USize(usize),
    Float(f64), 
    Array(Vec<ArrayTokenResult>), 
    Code(Vec<CodeTokenResult>),
    Variable(String)
}


#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceTokenPropertyType {
    Standard,
    Variable
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceToken {
    Control(String),
    EndControl(String),
    Property(SourceTokenPropertyType, String),
    PropertyValue(SourceTokenPropertyValue)
}

pub type SourceTokenResult = Result<SourceToken, SourceTokenError>;
pub type SourceTokenOption = Option<SourceTokenResult>;