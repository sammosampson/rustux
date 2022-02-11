mod strategies;
mod imports;
mod visiting;

pub use strategies::*;
pub use imports::*;
pub use visiting::*;

use crate::prelude::*;

pub fn build_streams(
    changes: &SourceChanges,
    source_files: &mut SourceFiles,
    stream_lookup: &mut AbstractSyntaxTokenStreamLookup
) -> Option<SourceLocation> {
    let mut root_location = None;
    for location in changes.iter() {
        let stream = build_stream(source_files.lookup(location).unwrap());
        if stream.contains_root() {
            root_location = Some(location.clone());
        } 
        stream_lookup.insert(location.clone(), stream);       
    }
    root_location
}

fn build_stream(source_text: &str) -> AbstractSyntaxTokenStream {
    let source_tokenizer = SourceTokenizer::from_string(source_text);
    let navigator = SourceTokenVisitationNavigator::from_source(source_tokenizer);
    let mut ast_build_visitor = create_ast_token_visitor();
    
    navigator.accept(&mut ast_build_visitor);
    ast_build_visitor.ast()        
}