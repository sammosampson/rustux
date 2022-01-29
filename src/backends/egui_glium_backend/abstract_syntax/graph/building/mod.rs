
mod strategies;
use strategies::*;

use crate::prelude::*;

pub struct AbstractSyntaxGraphBuilder {
    ast: AbstractSyntaxGraph,
    strategy: Box<dyn BuildAbstractSyntaxGraphStreamStrategy>,
    current_node: AbstractSyntaxGraphNodeId
}

impl Default for AbstractSyntaxGraphBuilder {
    fn default() -> Self {
        Self { 
            ast: Default::default(),
            strategy: Box::new(EmptyBuildAbstractSyntaxGraphStreamStrategy), 
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
        match node_type {
            AbstractSyntaxTokenType::Root =>
                self.strategy = Box::new(RootBuildAbstractSyntaxGraphStreamStrategy),
            AbstractSyntaxTokenType::For => 
                self.strategy = Box::new(ForBuildAbstractSyntaxGraphStreamStrategy),
            node_type => 
                self.strategy = Box::new(StandardBuildAbstractSyntaxGraphStreamStrategy(*node_type)),
        }
        self.current_node = self.strategy.start_node(self.current_node, &mut self.ast);
    }

    fn property(&mut self, property: &AbstractSyntaxTokenProperty) {
        self.strategy.property(self.current_node, property.clone(), &mut self.ast);
    }

    fn end_node(&mut self, _node_type: &AbstractSyntaxTokenType) {
        self.current_node = self.strategy.end_node(self.current_node, &mut self.ast);
    }
}