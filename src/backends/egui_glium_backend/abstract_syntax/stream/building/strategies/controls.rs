use crate::prelude::*;

#[derive(Default)]
pub struct ControlReferenceBuildAbstractSyntaxTokenStreamStrategy(pub String, pub Vec<ControlArgument>);

impl BuildAbstractSyntaxTokenStreamStrategy for ControlReferenceBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports) {
        if let Some(path) = self.get_import_path(imports) {
            start_control_reference_node(ast);
            create_name_property(ast, self.0.clone());
            create_path_property(ast, path.clone());
        } else {
            start_unknown_node(ast);
        }
    }

    fn child_control(&self, ast: &mut AbstractSyntaxTokenStream) {
        self.create_control_argments_property(ast);
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports) {
        if self.get_import_path(imports).is_some() {
            end_control_reference_node(ast);
        } else {
            end_unknown_node(ast);
        }
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(
        &mut self, 
        property: &CurrentProperty, 
        property_value: &SourceTokenPropertyValue, 
        ast: &mut AbstractSyntaxTokenStream, 
        _imports: &mut SourceImports
    ) {
        match property {
            CurrentProperty::Standard(property) => self.parse_control_argument(property, property_value, ast),
            CurrentProperty::Variable(variable) => add_unknown_property_error(ast, variable.clone()),
            _ => {},
        }
    }
}

impl ControlReferenceBuildAbstractSyntaxTokenStreamStrategy {
    fn get_import_path<'a>(&self, imports: &'a SourceImports) -> Option<&'a String> {
        imports.get_path(&self.0)
    }   

    fn create_control_argments_property(&self, ast: &mut AbstractSyntaxTokenStream) {
        if self.1.len() > 0 {
            ast.property(create_ast_property(
                AbstractSyntaxPropertyType::ControlArguments, 
                AbstractSyntaxPropertyValue::ControlArguments(create_control_arguments(self.1.clone()))));
        }
    }   

    fn parse_control_argument(&mut self, property: &String, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream) {
        match ControlArgument::parse(property.clone(), property_value.clone()) {
            Ok(value) => self.1.push(value),
            Err(error) => ast.add_error(error),
        }
    }
}

fn start_control_reference_node(ast: &mut AbstractSyntaxTokenStream) {
    ast.start_node(AbstractSyntaxControlType::ControlReference)
}

fn start_unknown_node(ast: &mut AbstractSyntaxTokenStream) {
    ast.start_node(AbstractSyntaxControlType::Unknown);
}

fn end_control_reference_node(ast: &mut AbstractSyntaxTokenStream) {
    ast.end_node(AbstractSyntaxControlType::ControlReference);
}

fn end_unknown_node(ast: &mut AbstractSyntaxTokenStream) {
    ast.end_node(AbstractSyntaxControlType::Unknown);
}

fn create_name_property(ast: &mut AbstractSyntaxTokenStream, name: String) {
    ast.property(create_ast_property(
        AbstractSyntaxPropertyType::Name, 
        AbstractSyntaxPropertyValue::String(name)));
}

fn create_path_property(ast: &mut AbstractSyntaxTokenStream, path: String) {
    ast.property(create_ast_property(
        AbstractSyntaxPropertyType::Path, 
        AbstractSyntaxPropertyValue::String(path)));
}

fn add_unknown_property_error(ast: &mut AbstractSyntaxTokenStream, property: String) {
    ast.add_error(AbstractSyntaxTokenError::UnknownProperty(property));
}
