
use crate::prelude::*;

#[derive(Default)]
pub struct ForBuildAbstractSyntaxGraphStreamStrategy {
    variable: Option<String>,
    range: Option<USizeRange>,
    current_position: usize
}

impl BuildAbstractSyntaxGraphStreamStrategy for ForBuildAbstractSyntaxGraphStreamStrategy {
    fn start_node(&mut self, parent: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.add_child_node(parent, AbstractSyntaxTokenType::Container)
    }

    fn end_node(&mut self, node: AbstractSyntaxGraphNodeId, ast: &mut AbstractSyntaxGraph) -> AbstractSyntaxGraphNodeId {
        ast.get_parent(node)
    }

    fn property(&mut self, _node: AbstractSyntaxGraphNodeId, property: AbstractSyntaxTokenProperty, _ast: &mut AbstractSyntaxGraph) {
        match property {
            AbstractSyntaxTokenProperty::USizeRangeVariable(variable, range) => {
                self.variable = Some(variable);
                self.current_position = range.lower_bound();
                self.range = Some(range);
            },
            _ => panic!(),
        }
    }

    fn start_child_node(&mut self, ast: &mut AbstractSyntaxGraph) {
        if let Some(variable) = &self.variable {
            ast.data_context().set_variable(variable.clone(), VariablePropertyType::Usize(self.current_position))
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