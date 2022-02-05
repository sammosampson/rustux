
use crate::prelude::*;

#[derive(Default)]
pub struct ForEachBuildAbstractSyntaxGraphStreamStrategy {
    variable_items: Option<(String, Vec<AbstractSyntaxPropertyValue>)>,
    current_position: usize
}

impl BuildAbstractSyntaxGraphStreamStrategy for ForEachBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, AbstractSyntaxControlType::Container)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) {
        let (variable, function) = property.value().get_function_variable_value().unwrap();
        self.current_position = 0;
        let function = context.replace_variable_data_in_function(&function).unwrap();
        if let AbstractSyntaxPropertyValue::Array(items) = context.run_selector_function(&function).unwrap() {
            self.variable_items = Some((variable, items));
        }
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) {
        if let Some((variable, items)) = &self.variable_items {
            if items.len() == 0 {
                return;
            }
            context.set_variable(variable.clone(), items[self.current_position].clone());
        }
    }

    fn end_child_node(&mut self) -> EndNodeAction {
        if let Some((_variable, items)) = &self.variable_items {
            self.current_position += 1;
            if self.current_position <= items.len() {
                return EndNodeAction::Repeat
            }
        }
        EndNodeAction::Continue
    }
}

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