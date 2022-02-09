use crate::prelude::*;

pub fn create_source_lookup() -> SourceLookup {
    SourceLookup::new()
}

pub type SourceLookup = HashMap<SourceLocation, String>;