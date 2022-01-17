
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

pub fn create_source_location_lookup() -> SourceLocationLookup {
    SourceLocationLookup::new()
}

pub type SourceLocationLookup = HashMap<Entity, SourceLocation>;

pub fn create_source_entity_lookup() -> SourceEntityLookup {
    SourceEntityLookup::new()
}

pub type SourceEntityLookup = HashMap<SourceLocation, Entity>;

pub fn create_source_tokens_lookup() -> SourceTokensLookup {
    SourceTokensLookup::new()
}

pub type SourceTokensLookup = HashMap<Entity, Vec<AbstractSyntaxTokenResult>>;


#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFile {
}


#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileRoot {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourceImplementation {
    pub source_file_entity: Entity
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileParsed {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileChange {
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileCreation {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileInitialRead {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileRemoval {
}


#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rebuild {
}

