
mod strategies;
pub use strategies::*;

use crate::prelude::*;

pub struct AbstractSyntaxGraphBuilder {
    ast: AbstractSyntaxGraph,
    strategies: Vec<Box<dyn BuildAbstractSyntaxGraphStreamStrategy>>,
    current_node: AbstractSyntaxGraphNodeId
}

impl Default for AbstractSyntaxGraphBuilder {
    fn default() -> Self {
        Self { 
            ast: Default::default(),
            strategies: vec!(), 
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
        let strategy = get_strategy(node_type);
        self.current_node = strategy.start_node(self.current_node, &mut self.ast);
        self.strategies.push(strategy);
    }

    fn property(&mut self, property: &AbstractSyntaxTokenProperty) {
        self.strategies.last().unwrap().property(self.current_node, property.clone(), &mut self.ast);
    }

    fn end_node(&mut self, _node_type: &AbstractSyntaxTokenType) {
        let strategy = self.strategies.pop().unwrap();
        self.current_node = strategy.end_node(self.current_node, &mut self.ast);
    }
}

fn get_strategy(node_type: &AbstractSyntaxTokenType) -> Box<dyn BuildAbstractSyntaxGraphStreamStrategy> {
    match node_type {
        AbstractSyntaxTokenType::Root =>
            Box::new(RootBuildAbstractSyntaxGraphStreamStrategy),
        AbstractSyntaxTokenType::For => 
            Box::new(ForBuildAbstractSyntaxGraphStreamStrategy),
        node_type => 
            Box::new(StandardBuildAbstractSyntaxGraphStreamStrategy(*node_type)),
    }
}