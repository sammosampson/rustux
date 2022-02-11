mod properties;
mod types;
mod linking;

pub use properties::*;
pub use types::*;
pub use linking::*;

use crate::prelude::*;

#[derive(Default)]
pub struct AbstractSyntax {
    graph: AbstractSyntaxGraph,
    linked_stream: AbstractSyntaxTokenStream,
    stream_lookup: AbstractSyntaxTokenStreamLookup,
    root_location: Option<SourceLocation>,
}

impl AbstractSyntax {
    pub fn build(&mut self, changes: &SourceChanges, source_files: &mut SourceFiles, context: &mut DataContext) {
        if !changes.is_empty() {
            self.build_and_link_streams(changes, source_files);
        }
        self.graph = build_graph(context, &mut self.linked_stream);

    }

    fn build_and_link_streams(&mut self, changes: &SourceChanges, source_files: &mut SourceFiles) {
        if let Some(root_location) = build_streams(changes, source_files, &mut self.stream_lookup) {
            self.root_location = Some(root_location);
        }
        if let Some(root_location) = &self.root_location {
            self.linked_stream = link_streams(root_location.clone(), root_location.clone(), &self.stream_lookup);
        } else{
            panic!("No root found")
        }
    }    

    pub fn graph(&self) ->  &AbstractSyntaxGraph {
        &self.graph
    }
}