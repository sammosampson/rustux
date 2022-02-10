use crate::prelude::*;

#[derive(Default)]
pub struct ControlBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for ControlBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        ast.start_node(AbstractSyntaxControlType::Empty);
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        ast.end_node(AbstractSyntaxControlType::Empty);
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(
        &self, 
        _property: &CurrentProperty, 
        _property_value: &SourceTokenPropertyValue, 
        _ast: &mut AbstractSyntaxTokenStream, 
        _imports: &mut SourceImports
    ) {
    }
}

#[derive(Default)]
pub struct ControlReferenceBuildAbstractSyntaxTokenStreamStrategy(pub String);

impl BuildAbstractSyntaxTokenStreamStrategy for ControlReferenceBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports) {
        if let Some(path) = imports.get_path(&self.0) {
            ast.start_node(AbstractSyntaxControlType::Control);
            ast.property(create_ast_property(
                AbstractSyntaxPropertyType::Name, 
                AbstractSyntaxPropertyValue::String(self.0.clone())));
            ast.property(create_ast_property(
                AbstractSyntaxPropertyType::Path, 
                AbstractSyntaxPropertyValue::String(path.clone())));
            } else {
            ast.start_node(AbstractSyntaxControlType::Unknown);
        }
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports) {
        if imports.get_path(&self.0).is_some() {
            ast.end_node(AbstractSyntaxControlType::Control);
        } else {
            ast.end_node(AbstractSyntaxControlType::Unknown);
        }
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(
        &self, 
        _property: &CurrentProperty, 
        _property_value: &SourceTokenPropertyValue, 
        _ast: &mut AbstractSyntaxTokenStream, 
        _imports: &mut SourceImports
    ) {
    }
}

