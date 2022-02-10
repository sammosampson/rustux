
use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SourceLocation {
    pub location: Option<String>
}

impl SourceLocation {
    pub fn to_relative_location(&self, relative_location: &str) -> Result<SourceLocation, SourceLocationError> {
        self.to_path_buf()
            .parent().unwrap()
            .join(relative_location)
            .to_canonicalised_source_location()
        
    }
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SourceLocationError {
    DoesNotExist
}


pub trait SourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError>;
}

