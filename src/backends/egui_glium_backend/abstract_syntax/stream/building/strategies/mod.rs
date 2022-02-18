mod imports;
mod controls;
mod standard;
mod looping;
mod variable_assigment;

pub use imports::*;
pub use controls::*;
pub use standard::*;
pub use looping::*;
pub use variable_assigment::*;

use crate::prelude::*;

pub trait BuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports);
    fn child_control(&self, ast: &mut AbstractSyntaxTokenStream);
    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream, imports: &SourceImports);
    fn property(&self, property: &CurrentProperty, ast: &mut AbstractSyntaxTokenStream);
    fn property_value(
        &mut self,
        property: &CurrentProperty,
        property_value: &SourceTokenPropertyValue,
        ast: &mut AbstractSyntaxTokenStream,
        imports: &mut SourceImports);
    
}

pub struct EmptyBuildAbstractSyntaxTokenStreamStrategy;

impl BuildAbstractSyntaxTokenStreamStrategy for EmptyBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, _ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        panic!()
    }

    fn child_control(&self, ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
        panic!()
    }

    fn property_value(
        &mut self, 
        _property: &CurrentProperty, 
        _property_value: &SourceTokenPropertyValue, 
        _ast: &mut AbstractSyntaxTokenStream, 
        _imports: &mut SourceImports
    ) {
        panic!()
    }

    fn end_control(&self, _ast: &mut AbstractSyntaxTokenStream, _imports: &SourceImports) {
        panic!()
    }
}