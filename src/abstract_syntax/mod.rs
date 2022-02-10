mod properties;
mod types;

pub use properties::*;
pub use types::*;

use crate::prelude::*;

#[derive(Default)]
pub struct AbstractSyntax {
    graph: AbstractSyntaxGraph,
    stream_lookup: AbstractSyntaxTokenStreamLookup,
    root_location: Option<SourceLocation>,
}

impl AbstractSyntax {
    pub fn build(&mut self, changes: &SourceChanges, source_files: &mut SourceFiles, context: &mut DataContext) {
        self.build_streams(changes, source_files);
        if let Some(root_location) = &self.root_location {
            self.graph = self.build_graph(context, root_location);
        }
    }
    
    fn build_graph(&self, context: &mut DataContext, root_location: &SourceLocation) ->  AbstractSyntaxGraph {
        let ast_stream = self.stream_lookup.get(root_location).unwrap(); 
        let mut graph_builder = AbstractSyntaxGraphBuilder::default();
        ast_stream.accept(&mut graph_builder, context);
        graph_builder.ast()
    }

    fn build_streams(&mut self, changes: &SourceChanges, source_files: &mut SourceFiles) {
        for location in changes.iter() {
            let stream = self.build_stream(source_files.lookup(location).unwrap());
            if stream.contains_root() {
                self.root_location = Some(location.clone());
            } 
            self.stream_lookup.insert(location.clone(), stream);       
        }
    }

    fn build_stream(&self, source_text: &str) -> AbstractSyntaxTokenStream {
        let source_tokenizer = SourceTokenizer::from_string(source_text);
        let navigator = SourceTokenVisitationNavigator::from_source(source_tokenizer);
        let mut ast_build_visitor = create_ast_token_visitor();
        
        navigator.accept(&mut ast_build_visitor);
        ast_build_visitor.ast()        
    }

    pub fn graph(&self) ->  &AbstractSyntaxGraph {
        &self.graph
    }
}