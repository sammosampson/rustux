
use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnusedPropertyType,
    UnknownProperty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractSyntaxNodeType {
    Unknown,
    Root,
    Import,
    Control,
    ControlImplementation,
    CompleteControl,
}

#[derive(Debug, Copy, Clone)]
pub struct AbstractSyntaxNode {
    node_type: AbstractSyntaxNodeType
}

pub struct AbstractSyntaxTree;

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxNode, AbstractSyntaxTokenError>;
pub type AbstractSyntaxTokenOption = Option<AbstractSyntaxTokenResult>;

pub struct AbstractSyntaxTokenizer<I> where I : Iterator<Item=SourceTokenResult> {
    source_token_iterator: I,
    current_property: String
}

impl <'a, I> Iterator for AbstractSyntaxTokenizer<I> where I : Iterator<Item=SourceTokenResult> {
    type Item = AbstractSyntaxTokenResult;
    fn next(&mut self) -> AbstractSyntaxTokenOption {
        if let Some(token) = self.source_token_iterator.next() {
            println!("{:?}", token);
            return Some(Ok(AbstractSyntaxNode { node_type: AbstractSyntaxNodeType::CompleteControl }))
        }
        None
    }
}

impl <I> AbstractSyntaxTokenizer<I>  where I : Iterator<Item=SourceTokenResult> {
    pub fn from_source(source_token_iterator: I) -> Self {
        Self {
            source_token_iterator,
            current_property: String::from("")
        }
    }
}

pub fn contains_root(tokens: &Vec<AbstractSyntaxTokenResult>) -> bool {
    tokens
        .iter()
        .any(|token_result| { 
            match token_result {
                Ok(token) => token.node_type == AbstractSyntaxNodeType::Root,
                Err(_) => false
            }
        })
}