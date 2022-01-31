
use crate::prelude::*;

pub struct RootBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for RootBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, _parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_root()
    }

    fn end_node(&mut self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        EndNodeAction::Continue
    }
}
