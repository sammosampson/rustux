use crate::prelude::*;

#[derive(Debug, Default)]
pub struct BuildAbstractSyntaxSourceTokenVisitor {
    ast: AbstractSyntaxTokenStream
}

impl BuildAbstractSyntaxSourceTokenVisitor {
    pub fn ast(self) -> AbstractSyntaxTokenStream {
        self.ast
    }
}

impl SourceTokenVisitor for BuildAbstractSyntaxSourceTokenVisitor {
    fn token_error(&mut self, error: SourceTokenError) {
        self.ast.add_error(AbstractSyntaxTokenError::SourceTokenError(error))
    }

    fn control(&mut self, control_name: &str) {
        self.ast.start_node(match_control_name(control_name));
    }

    fn end_control(&mut self, control_name: &str) {
        self.ast.end_node(match_control_name(control_name));
    }
}

fn match_control_name(control_name: &str) -> AbstractSyntaxTokenType {
    match control_name {
        "root" => AbstractSyntaxTokenType::Root,
        "left-side-bar" => AbstractSyntaxTokenType::LeftSidebar,
        "right-side-bar" => AbstractSyntaxTokenType::RightSidebar,
        _ => AbstractSyntaxTokenType::Unknown
    }
}