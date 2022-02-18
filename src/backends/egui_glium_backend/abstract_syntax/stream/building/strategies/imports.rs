use crate::prelude::*;

#[derive(Default)]
pub struct ImportBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for ImportBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        ast.start_node(AbstractSyntaxControlType::Empty);
    }

    fn child_control(&self, ast: &mut AbstractSyntaxTokenStream) {
    }
    
    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        ast.end_node(AbstractSyntaxControlType::Empty);
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(
        &mut self, 
        property: &CurrentProperty, 
        property_value: &SourceTokenPropertyValue, 
        ast: &mut AbstractSyntaxTokenStream, 
        imports: &mut SourceImports
    ) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(current_property_name) => {
                if let Err(error) = store_import(current_property_name, property_value, imports) {
                    ast.property_error(error);
                }
            },
            CurrentProperty::Variable(variable_name) =>
                ast.property_error(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())),
        }
    }
}

fn store_import(
    property_name: &str,
    property_value: &SourceTokenPropertyValue,
    imports: &mut SourceImports
)  -> Result<(), AbstractSyntaxTokenError> {
    match property_name {
        "name" => imports.push_name(property_value),
        "path" => imports.push_path(property_value),
        other => return Err(AbstractSyntaxTokenError::UnknownProperty(other.to_string()))
    }
}

