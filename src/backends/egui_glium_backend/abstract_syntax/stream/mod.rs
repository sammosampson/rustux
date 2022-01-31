
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
    fn token_error(&mut self, error: &AbstractSyntaxTokenError);
    fn start_node(&mut self, node_type: &AbstractSyntaxTokenType);
    fn property(&mut self, property: &AbstractSyntaxTokenProperty);
    fn end_node(&mut self, node_type: &AbstractSyntaxTokenType) -> EndNodeAction;
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
        for node_result in &self.0 {
            match node_result {
                Ok(node) => match node {
                    AbstractSyntaxToken::StartNode(node_type) => visitor.start_node(node_type),
                    AbstractSyntaxToken::Property(property) => visitor.property(property),
                    AbstractSyntaxToken::EndNode(node_type) =>
                        if visitor.end_node(node_type) == EndNodeAction::Repeat {

                        },
                },
                Err(error) => visitor.token_error(error),
            }
        }
    }
}