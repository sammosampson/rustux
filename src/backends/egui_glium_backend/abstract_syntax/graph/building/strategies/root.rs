
use crate::prelude::*;

pub struct RootBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for RootBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, _parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        let node = ast.add_root();
        context.scopes_mut().set(node);
        node
    }

    fn end_node(&mut self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        context.scopes_mut().pop();
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxProperty, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) {
        panic!()
    }

    fn end_child_node(&mut self, _context: &mut DataContext) -> EndNodeAction {
        EndNodeAction::Continue
    }
    
    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> StartNodeAction {
        StartNodeAction::Continue
    }
}
