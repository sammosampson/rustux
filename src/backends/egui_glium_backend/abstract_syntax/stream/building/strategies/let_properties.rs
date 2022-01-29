use crate::prelude::*;

pub struct LetBuildPropertyStrategy;

impl BuildPropertyStrategy for LetBuildPropertyStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.start_node(AbstractSyntaxTokenType::Let);
    }
    
    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(&self, property: &CurrentProperty, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(property_name) => ast.property_error(AbstractSyntaxTokenError::UnknownProperty(property_name.to_string())),
            CurrentProperty::Variable(variable_name) => match match_property_value(variable_name, property_value) {
                Ok(property) => ast.property(property),
                Err(error) => ast.property_error(error)
            },
        }
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.end_node(AbstractSyntaxTokenType::Let);
    }
}

fn match_property_value(variable_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxTokenProperty, AbstractSyntaxTokenError> {
    match property_value {
        SourceTokenPropertyValue::Code(tokens) => 
        Ok(AbstractSyntaxTokenProperty::FunctionVariable(variable_name.to_string(), Function::parse(tokens)?)),
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())) 
    }
}