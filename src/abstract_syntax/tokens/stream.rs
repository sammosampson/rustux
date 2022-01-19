
use crate::prelude::*;

pub fn create_abstract_syntax_token_stream_lookup() -> AbstractSyntaxTokenStreamLookup {
    AbstractSyntaxTokenStreamLookup::new()
}

pub type AbstractSyntaxTokenStreamLookup = HashMap<Entity, AbstractSyntaxTokenStream>;

pub trait AbstractSyntaxTokenStreamVisitor {
    fn token_error(&mut self, error: &AbstractSyntaxTokenError);
    fn start_node(&mut self, node_type: &AbstractSyntaxTokenType);
    fn end_node(&mut self, node_type: &AbstractSyntaxTokenType);
}

#[derive(Debug, Clone, Default)]
pub struct AbstractSyntaxTokenStream(Vec<AbstractSyntaxTokenResult>, bool);

impl AbstractSyntaxTokenStream {
    pub fn add_error(&mut self, error: AbstractSyntaxTokenError) {
        self.0.push(Err(error));
    }

    pub fn start_node(&mut self, node_type: AbstractSyntaxTokenType) {
        if node_type == AbstractSyntaxTokenType::Root {
            self.1 = true;
        }
        self.0.push(Ok(AbstractSyntaxToken::Start(node_type)));

    }

    pub fn end_node(&mut self, node_type: AbstractSyntaxTokenType) {
        self.0.push(Ok(AbstractSyntaxToken::End(node_type)));
    }

    pub fn contains_root(&self) -> bool {
        self.1
    }

    pub fn accept(&self, visitor: &mut impl AbstractSyntaxTokenStreamVisitor) {
        for node_result in &self.0 {
            match node_result {
                Ok(node) => match node {
                    AbstractSyntaxToken::Start(node_type) => visitor.start_node(node_type),
                    AbstractSyntaxToken::End(node_type) => visitor.end_node(node_type)
                },
                Err(error) => visitor.token_error(error),
            }
        }
    }
}