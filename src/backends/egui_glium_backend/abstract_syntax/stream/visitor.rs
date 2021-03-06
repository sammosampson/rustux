use crate::prelude::*;

#[derive(PartialEq, Eq)]
pub enum StartNodeAction {
    Continue,
    Prevent
}

#[derive(PartialEq, Eq)]
pub enum EndNodeAction {
    Continue,
    Repeat
}

pub trait AbstractSyntaxTokenStreamVisitor {
    fn start_node_with_repeat_possibility(&mut self, position: usize, node_type: &AbstractSyntaxControlType, context: &mut DataContext) {
        self.push_last_node_position(position);
        self.start_node(node_type, context);
    }

    fn end_node_with_repeat_check(&mut self, position: usize, node_type: &AbstractSyntaxControlType, context: &mut DataContext) -> Option<USizeRange> {
        if let Some(last_node_position) = self.pop_last_node_position() {
            if self.end_node(node_type, context) == EndNodeAction::Repeat {
                return Some(USizeRange::new(last_node_position, position));
            }
        }
        None
    }

    fn push_last_node_position(&mut self, position: usize);
    fn pop_last_node_position(&mut self) -> Option<usize>;
    fn start_node(&mut self, node_type: &AbstractSyntaxControlType, context: &mut DataContext);
    fn property(&mut self, property: &AbstractSyntaxProperty, context: &mut DataContext);
    fn end_node(&mut self, node_type: &AbstractSyntaxControlType, context: &mut DataContext) -> EndNodeAction;
    fn token_error(&mut self, error: &AbstractSyntaxTokenError);
}