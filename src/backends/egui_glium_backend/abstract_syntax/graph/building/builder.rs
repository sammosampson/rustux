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

    fn start_node(&mut self, node_type: &AbstractSyntaxControlType, context: &mut DataContext) {
        let mut action = StartNodeAction::Continue;

        if let Some(parent_strategy) = self.strategies.last_mut() {
            action = parent_strategy.start_child_node(&mut self.ast, context);
        }
        
        
        let mut strategy = get_strategy(node_type);
        
        if action == StartNodeAction::Prevent {
            strategy = Box::new(PreventBuildAbstractSyntaxGraphStreamStrategy);
        }

        self.current_node = strategy.start_node(self.current_node, &mut self.ast);
        self.strategies.push(strategy);
    }

    fn property(&mut self, property: &AbstractSyntaxProperty, context: &mut DataContext) {
        self.strategies.last_mut().unwrap().property(self.current_node, property.clone(), &mut self.ast, context);
    }

    fn end_node(&mut self, _node_type: &AbstractSyntaxControlType) -> EndNodeAction {
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

fn get_strategy(node_type: &AbstractSyntaxControlType) -> Box<dyn BuildAbstractSyntaxGraphStreamStrategy> {
    match node_type {
        AbstractSyntaxControlType::Root =>
            Box::new(RootBuildAbstractSyntaxGraphStreamStrategy),
        AbstractSyntaxControlType::For => 
            Box::new(ForBuildAbstractSyntaxGraphStreamStrategy::default()),
        AbstractSyntaxControlType::ForEach => 
            Box::new(ForEachBuildAbstractSyntaxGraphStreamStrategy::default()),
        AbstractSyntaxControlType::Let => 
            Box::new(LetBuildAbstractSyntaxGraphStreamStrategy::default()),
        node_type => 
            Box::new(StandardBuildAbstractSyntaxGraphStreamStrategy(*node_type)),
    }
}