use crate::prelude::*;

pub struct ForEachBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for ForEachBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.start_node(AbstractSyntaxControlType::ForEach);
    }
    
    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(&self, property: &CurrentProperty, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(property_name) => ast.property_error(AbstractSyntaxTokenError::UnknownProperty(property_name.to_string())),
            CurrentProperty::Variable(variable_name) => match match_for_each_property_value(variable_name, property_value) {
                Ok(property) => ast.property(property),
                Err(error) => ast.property_error(error)
            },
        }
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.end_node(AbstractSyntaxControlType::For);
    }
}

pub struct ForBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for ForBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.start_node(AbstractSyntaxControlType::For);
    }
    
    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(&self, property: &CurrentProperty, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(property_name) => ast.property_error(AbstractSyntaxTokenError::UnknownProperty(property_name.to_string())),
            CurrentProperty::Variable(variable_name) => match match_for_property_value(variable_name, property_value) {
                Ok(property) => ast.property(property),
                Err(error) => ast.property_error(error)
            },
        }
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.end_node(AbstractSyntaxControlType::For);
    }
}

fn match_for_each_property_value(variable_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxProperty, AbstractSyntaxTokenError> {
    match property_value {
        SourceTokenPropertyValue::Code(token_result) => 
        Ok(create_ast_property(
            AbstractSyntaxPropertyType::FunctionVariable, 
            AbstractSyntaxPropertyValue::FunctionVariable(variable_name.to_string(), Function::parse(token_result)?)
        )),
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())) 
    }
}

fn match_for_property_value(variable_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxProperty, AbstractSyntaxTokenError> {
    match property_value {
        SourceTokenPropertyValue::Array(token_result) => 
        Ok(create_ast_property(
            AbstractSyntaxPropertyType::USizeRangeVariable, 
            AbstractSyntaxPropertyValue::USizeRangeVariable(variable_name.to_string(), USizeRange::parse(token_result)?)
        )),
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())) 
    }
}