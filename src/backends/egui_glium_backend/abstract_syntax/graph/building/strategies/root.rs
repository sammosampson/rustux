
use crate::prelude::*;

pub struct RootBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for RootBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, _parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_root()
    }

    fn end_node(&self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }
}
