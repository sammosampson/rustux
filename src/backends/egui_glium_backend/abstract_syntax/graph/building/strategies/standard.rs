

use crate::prelude::*;

pub struct StandardBuildAbstractSyntaxGraphStreamStrategy(pub AbstractSyntaxControlType);

impl BuildAbstractSyntaxGraphStreamStrategy for StandardBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, self.0)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&mut self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) {
        let resolved_property = context.replace_variable_data_in_property(property).unwrap();
        ast.add_node_property(node, resolved_property);
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        EndNodeAction::Continue
    }
    
    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) {
    }
}
