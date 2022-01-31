mod root;
mod standard;
mod looping;

pub use root::*;
pub use standard::*;
pub use looping::*;

use crate::prelude::*;

pub trait BuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn property(&mut self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph);
    fn end_child_node(&mut self) -> EndNodeAction;
}

pub struct EmptyBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for EmptyBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, _parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn end_node(&mut self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        panic!()
    }
}