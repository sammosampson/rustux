mod root;
mod standard;
mod looping;

pub use root::*;
pub use standard::*;
pub use looping::*;

use crate::prelude::*;

pub trait BuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn end_node(&self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn property(&self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph);
}

pub struct EmptyBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for EmptyBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, _parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn end_node(&self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn property(&self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }
}