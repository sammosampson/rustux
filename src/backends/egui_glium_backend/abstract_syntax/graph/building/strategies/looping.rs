
use crate::prelude::*;
pub struct ForBuildAbstractSyntaxGraphStreamStrategy;

impl BuildAbstractSyntaxGraphStreamStrategy for ForBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn end_node(&self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        AbstractSyntaxGraphNodeId::default()
    }

    fn property(&self, node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, ast: &mut AbstractSyntaxGraph) {
        match property {
            AbstractSyntaxTokenProperty::UnsignedIntRangeVariable(variable, range) => todo!(),
            _ => panic!(),
        }
    }
}