mod root;
mod standard;
mod looping;
mod data;
mod scoping;

pub use root::*;
pub use standard::*;
pub use looping::*;
pub use looping::*;
pub use data::*;
pub use scoping::*;

use crate::prelude::*;

pub trait BuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> AbstractSyntaxGraphNodeId;
    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> AbstractSyntaxGraphNodeId;
    fn property(&mut self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty, ast: &mut AbstractSyntaxGraph, context: &mut DataContext);
    fn start_child_node(&mut self, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> StartNodeAction;
    fn end_child_node(&mut self, context: &mut DataContext) -> EndNodeAction;
}

pub struct EmptyBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for EmptyBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, AbstractSyntaxControlType::Container)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(
        &mut self, 
        _node: AbstractSyntaxGraphNodeId,
        _property: AbstractSyntaxProperty,
        _ast: &mut AbstractSyntaxGraph,
        _context: &mut DataContext
    ) {
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> StartNodeAction {
        StartNodeAction::Continue
    }

    fn end_child_node(&mut self, _context: &mut DataContext) -> EndNodeAction {
        EndNodeAction::Continue
    }
}

pub struct PreventBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for PreventBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        parent
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        node
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxProperty, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) {
    }

    fn end_child_node(&mut self, _context: &mut DataContext) -> EndNodeAction {
        EndNodeAction::Continue
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> StartNodeAction {
        StartNodeAction::Prevent
    }
}