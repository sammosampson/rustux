use crate::prelude::*;

pub struct LetBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for LetBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.start_node(AbstractSyntaxControlType::Let);
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
        ast.end_node(AbstractSyntaxControlType::Let);
    }
}

fn match_property_value(variable_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxProperty, AbstractSyntaxTokenError> {
    match property_value {
        SourceTokenPropertyValue::Code(tokens) => 
        Ok(create_ast_property(
            AbstractSyntaxPropertyType::FunctionVariable, 
            AbstractSyntaxPropertyValue::FunctionVariable(variable_name.to_string(), Function::parse(tokens)?)
        )),
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())) 
    }
}