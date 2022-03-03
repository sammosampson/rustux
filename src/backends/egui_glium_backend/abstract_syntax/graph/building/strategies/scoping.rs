
use crate::prelude::*;

#[derive(Default)]
pub struct ScopeBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for ScopeBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(
        &mut self,
        parent: AbstractSyntaxGraphNodeId,
        ast: &mut AbstractSyntaxGraph,
        context: &mut DataContext
    ) -> AbstractSyntaxGraphNodeId {
        let node = ast.add_child_node(parent, AbstractSyntaxControlType::Scope);
        context.scopes_mut().set(node);
        node
    }

    fn end_node(
        &mut self,
        node: AbstractSyntaxGraphNodeId,
        ast: &mut AbstractSyntaxGraph,
        context: &mut DataContext
    ) -> AbstractSyntaxGraphNodeId {
        context.scopes_mut().pop();
        ast.get_parent(node)
    }

    fn property(
        &mut self, 
        _node: AbstractSyntaxGraphNodeId,
        property: AbstractSyntaxProperty,
        _ast: &mut AbstractSyntaxGraph,
        context: &mut DataContext
    ) {
        match property.property_type() {
            AbstractSyntaxPropertyType::ControlArguments => context
                .current_scope_mut()
                .set_control_args(property.value().get_control_arguments_value().unwrap()),
            _ => {}
        } 
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> StartNodeAction {
        StartNodeAction::Continue
    }

    fn end_child_node(&mut self, _context: &mut DataContext) -> EndNodeAction {
        EndNodeAction::Continue
    }
}