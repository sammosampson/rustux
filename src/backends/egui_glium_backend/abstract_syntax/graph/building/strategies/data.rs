
use crate::prelude::*;

#[derive(Default)]
pub struct LetBuildAbstractSyntaxGraphStreamStrategy {
    function_variable: Option<(String, Function)>,
}

impl BuildAbstractSyntaxGraphStreamStrategy for LetBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, AbstractSyntaxControlType::Container)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph, _context: &mut DataContext) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(
        &mut self, 
        _node: AbstractSyntaxGraphNodeId,
        property: AbstractSyntaxProperty,
        _ast: &mut AbstractSyntaxGraph,
        _context: &mut DataContext
    ) {
        self.function_variable = Some(property.value().get_function_variable_value().unwrap());
    }

    fn start_child_node(&mut self, _ast: &mut AbstractSyntaxGraph, context: &mut DataContext) -> StartNodeAction {
        if let Some((variable, function)) = &self.function_variable {
            
            let function = context
                .replace_variable_data_in_function(function)
                .unwrap();

            let function_value = context
                .run_selector_function(&function)
                .unwrap();
            
                context.current_scope_mut().variables_mut().set(variable.clone(), function_value);
        }
        StartNodeAction::Continue
    }

    fn end_child_node(&mut self, context: &mut DataContext) -> EndNodeAction {
        if let Some((variable, _function)) = &self.function_variable {
            context.current_scope_mut().variables_mut().remove(variable);
        }
        EndNodeAction::Continue
    }
}