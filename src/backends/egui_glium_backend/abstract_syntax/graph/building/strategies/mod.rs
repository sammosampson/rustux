mod root;
mod standard;
mod looping;
mod data;

pub use root::*;
pub use standard::*;
pub use looping::*;
pub use looping::*;
pub use data::*;

use crate::prelude::*;

pub trait BuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn property(&mut self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty, ast: &mut AbstractSyntaxGraph, context: &mut DataContext);
    fn start_child_node(&mut self, ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> StartNodeAction;
    fn end_child_node(&mut self, context: &mut DataContext) -> EndNodeAction;
}

pub struct EmptyBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for EmptyBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, _parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn end_node(&mut self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxProperty, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) {
        panic!()
    }

    fn end_child_node(&mut self, _context: &mut DataContext) -> EndNodeAction {
        panic!()
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> StartNodeAction {
        panic!()
    }
}

pub struct PreventBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for PreventBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        parent
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
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