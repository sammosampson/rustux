
mod strategies;
mod builder;

pub use strategies::*;
pub use builder::*;

use crate::prelude::*;

pub fn build_graph(
    context: &mut DataContext,
    linked_stream: &mut AbstractSyntaxTokenStream
) ->  AbstractSyntaxGraph {
    let mut graph_builder = AbstractSyntaxGraphBuilder::default();
    linked_stream.accept(&mut graph_builder, context);
    graph_builder.ast()
}