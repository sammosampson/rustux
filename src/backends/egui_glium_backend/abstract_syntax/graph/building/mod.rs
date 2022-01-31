
mod strategies;
pub use strategies::*;

use crate::prelude::*;

pub struct AbstractSyntaxGraphBuilder {
    ast: AbstractSyntaxGraph,
    strategies: Vec<Box<dyn BuildAbstractSyntaxGraphStreamStrategy>>,
    positions: Vec<usize>,
    current_node: AbstractSyntaxGraphNodeId
}

impl Default for AbstractSyntaxGraphBuilder {
    fn default() -> Self {
        Self { 
            ast: Default::default(),
            strategies: vec!(), 
            positions: vec!(), 
            current_node: Default::default()
        }
    }
}

impl AbstractSyntaxGraphBuilder {
    pub fn ast(self) -> AbstractSyntaxGraph {
        self.ast
    }
}

impl AbstractSyntaxTokenStreamVisitor for AbstractSyntaxGraphBuilder {
    fn token_error(&mut self, error: &AbstractSyntaxTokenError) {
        panic!("{:?}", error)
    }

    fn start_node(&mut self, node_type: &AbstractSyntaxTokenType) {
        let mut strategy = get_strategy(node_type);
        self.current_node = strategy.start_node(self.current_node, &mut self.ast);
        self.strategies.push(strategy);
    }

    fn property(&mut self, property: &AbstractSyntaxTokenProperty) {
        self.strategies.last_mut().unwrap().property(self.current_node, property.clone(), &mut self.ast);
    }

    fn end_node(&mut self, _node_type: &AbstractSyntaxTokenType) -> EndNodeAction {
        let mut strategy = self.strategies.pop().unwrap();
        let ending_node = self.current_node;
        self.current_node = strategy.end_node(ending_node, &mut self.ast);
        
        let mut end_node_action = EndNodeAction::Continue;

        if let Some(parent_strategy) = self.strategies.last_mut() {
            end_node_action = parent_strategy.end_child_node();
        }

        end_node_action
    }

    fn push_last_node_position(&mut self, position: usize) {
        self.positions.push(position);
    }

    fn pop_last_node_position(&mut self) -> Option<usize> {
        self.positions.pop()
    }
}

fn get_strategy(node_type: &AbstractSyntaxTokenType) -> Box<dyn BuildAbstractSyntaxGraphStreamStrategy> {
    match node_type {
        AbstractSyntaxTokenType::Root =>
            Box::new(RootBuildAbstractSyntaxGraphStreamStrategy),
        AbstractSyntaxTokenType::For => 
            Box::new(ForBuildAbstractSyntaxGraphStreamStrategy::default()),
        node_type => 
            Box::new(StandardBuildAbstractSyntaxGraphStreamStrategy(*node_type)),
    }
}