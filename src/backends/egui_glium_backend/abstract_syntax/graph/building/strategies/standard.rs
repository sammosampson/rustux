

use crate::prelude::*;

pub struct StandardBuildAbstractSyntaxGraphStreamStrategy(pub AbstractSyntaxTokenType);

impl BuildAbstractSyntaxGraphStreamStrategy for StandardBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, self.0)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&mut self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph) {
        ast.add_node_property(node, property);
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        EndNodeAction::Continue
    }
}
