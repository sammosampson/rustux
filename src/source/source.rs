
use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SourceLocation {
    pub location: Option<String>
}

impl From<&str> for SourceLocation {
    fn from(from: &str) -> Self {
        Self { location: Some(from.to_owned()) }
    }
}

impl From<SourceLocation> for String {
    fn from(from: SourceLocation) -> Self {
        match from.location {
            Some(value) => value.to_owned(),
            None => String::default()
        }
    }
}

#[derive(Debug)]
pub enum SourceReaderError {
    ErrorReadingSource
}

#[derive(Debug)]
pub enum SourceLocationError {
    DoesNotExist
}


pub trait SourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError>;
    fn get_relative_source_location(&self, from: &SourceLocation, relative_location: &str) -> Result<SourceLocation, SourceLocationError>;
}

