use crate::prelude::*;

pub trait BuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn end_node(&self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId;
    fn property(&self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph);
}

pub struct EmptyBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for EmptyBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, _parent: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn end_node(&self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        panic!()
    }

    fn property(&self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }
}

pub struct RootBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for RootBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, _parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_root()
    }

    fn end_node(&self, _node: AbstractSyntaxGraphNodeId, _ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&self, _node: AbstractSyntaxGraphNodeId, _property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        panic!()
    }
}

pub struct StandardBuildAbstractSyntaxGraphStreamStrategy(pub AbstractSyntaxTokenType);

impl BuildAbstractSyntaxGraphStreamStrategy for StandardBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, self.0)
    }

    fn end_node(&self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph) {
        ast.add_node_property(node, property);
    }
}

pub struct ForBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for ForBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn end_node(&self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph) {
    }
}
