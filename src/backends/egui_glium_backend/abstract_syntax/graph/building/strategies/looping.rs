
use crate::prelude::*;

#[derive(Default)]
pub struct ForBuildAbstractSyntaxGraphStreamStrategy {
    variable: Option<String>,
    range: Option<USizeRange>,
    current_position: usize
}

impl BuildAbstractSyntaxGraphStreamStrategy for ForBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, AbstractSyntaxControlType::Container)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty, _ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) {
        let (variable, range) = property.value().get_usize_range_variable_value().unwrap();
        self.variable = Some(variable);
        self.current_position = range.lower_bound();
        self.range = Some(range);
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) {
        if let Some(variable) = &self.variable {
            context.set_variable(variable.clone(), AbstractSyntaxPropertyValue::USize(self.current_position));
        }
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        if let Some(range) = &self.range {
            self.current_position += 1;
            if self.current_position <= range.upper_bound() {
                return EndNodeAction::Repeat
            }
        }
        EndNodeAction::Continue
    }
}