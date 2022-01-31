
mod building;

pub use building::*;

use crate::prelude::*;

pub fn create_abstract_syntax_token_stream_lookup() -> AbstractSyntaxTokenStreamLookup {
    AbstractSyntaxTokenStreamLookup::new()
}

pub type AbstractSyntaxTokenStreamLookup = HashMap<Entity, AbstractSyntaxTokenStream>;

#[derive(PartialEq, Eq)]
pub enum EndNodeAction {
    Continue,
    Repeat
}

pub trait AbstractSyntaxTokenStreamVisitor {
    fn start_node_with_repeat_possibility(&mut self, position: usize, node_type: &AbstractSyntaxTokenType) {
        self.push_last_node_position(position);
        self.start_node(node_type)
    }

    fn end_node_with_repeat_check(&mut self, position: usize, node_type: &AbstractSyntaxTokenType) -> Option<USizeRange> {
        if let Some(last_node_position) = self.pop_last_node_position() {
            if self.end_node(node_type) == EndNodeAction::Repeat {
                return Some(USizeRange::new(last_node_position, position));
            }
        }
        None
    }

    fn push_last_node_position(&mut self, position: usize);
    fn pop_last_node_position(&mut self) -> Option<usize>;
    fn start_node(&mut self, node_type: &AbstractSyntaxTokenType);
    fn property(&mut self, property: &AbstractSyntaxTokenProperty);
    fn end_node(&mut self, node_type: &AbstractSyntaxTokenType) -> EndNodeAction;
    fn token_error(&mut self, error: &AbstractSyntaxTokenError);
}

#[derive(Debug, Clone, Default)]
pub struct AbstractSyntaxTokenStream(Vec<AbstractSyntaxTokenResult>, bool);

impl AbstractSyntaxTokenStream {
    pub fn add_error(&mut self, error: AbstractSyntaxTokenError) {
        self.0.push(Err(error));
    }

    pub fn start_node(&mut self, node_type: AbstractSyntaxTokenType) {
        println!("{:?}", node_type);
        if node_type == AbstractSyntaxTokenType::Root {
            self.1 = true;
        }
        self.0.push(Ok(AbstractSyntaxToken::StartNode(node_type)));

    }

    pub fn property(&mut self, property: AbstractSyntaxTokenProperty) {
        println!("prop {:?}", property);
        self.0.push(Ok(AbstractSyntaxToken::Property(property)));
    }

    pub fn property_error(&mut self, error: AbstractSyntaxTokenError) {
        self.0.push(Err(error));
    }

    pub fn end_node(&mut self, node_type: AbstractSyntaxTokenType) {
        self.0.push(Ok(AbstractSyntaxToken::EndNode(node_type)));
    }

    pub fn contains_root(&self) -> bool {
        self.1
    }

    pub fn accept(&self, visitor: &mut impl AbstractSyntaxTokenStreamVisitor) {
        for position in 0..self.0.len() {
            self.accept_node(position, visitor);
        }
    }

    fn accept_node(&self, position: usize, visitor: &mut impl AbstractSyntaxTokenStreamVisitor) {
        let node_result = &self.0[position];

        match node_result {
            Ok(node) => match node {
                AbstractSyntaxToken::StartNode(node_type) => visitor.start_node_with_repeat_possibility(position, node_type),
                AbstractSyntaxToken::Property(property) => visitor.property(property),
                AbstractSyntaxToken::EndNode(node_type) =>
                    if let Some(range) = visitor.end_node_with_repeat_check(position, node_type) {
                        for child_position in RangeInclusive::<usize>::from(&range) {
                            self.accept_node(child_position, visitor);
                        }
                    },
            },
            Err(error) => visitor.token_error(error),
        }
    }
}

