use crate::prelude::*;

pub fn create_abstract_syntax_token_stream_lookup() -> AbstractSyntaxTokenStreamLookup {
    AbstractSyntaxTokenStreamLookup::new()
}

pub type AbstractSyntaxTokenStreamLookup = HashMap<Entity, AbstractSyntaxTokenStream>;