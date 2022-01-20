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

    fn property(&mut self, property_name: &str, property_value: &SourceTokenPropertyValue) {
        match match_property(property_name, property_value) {
            Ok(property) => self.ast.property(property),
            Err(error) => self.ast.property_error(error),
        }
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
        "horizontal" => AbstractSyntaxTokenType::Horizontal,
        "vertical" => AbstractSyntaxTokenType::Vertical,
        "label" => AbstractSyntaxTokenType::Label,
        _ => AbstractSyntaxTokenType::Unknown
    }
}

fn match_property(property_name: &str, value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxTokenProperty, AbstractSyntaxTokenError> {
    match property_name {
        "name" => {
            match value {
                SourceTokenPropertyValue::String(v) => Ok(AbstractSyntaxTokenProperty::Name(v.clone())),
                _ => Err(AbstractSyntaxTokenError::UnknownProperty) 
            }
        },
        "text" => {
            match value {
                SourceTokenPropertyValue::String(v) => Ok(AbstractSyntaxTokenProperty::Text(v.clone())),
                _ => Err(AbstractSyntaxTokenError::UnknownProperty) 
            }
        },
        _ => Err(AbstractSyntaxTokenError::UnknownProperty) 
    }
}