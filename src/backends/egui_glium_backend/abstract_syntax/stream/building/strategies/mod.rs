mod standard_properties;
mod for_properties;
mod let_properties;

pub use standard_properties::*;
pub use for_properties::*;
pub use let_properties::*;

use crate::prelude::*;

pub trait BuildPropertyStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream);
    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream);
    fn property(&self, property: &CurrentProperty, ast: &mut AbstractSyntaxTokenStream);
    fn property_value(&self, property: &CurrentProperty, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream);
    
}

pub struct EmptyBuildPropertyStrategy;

impl BuildPropertyStrategy for EmptyBuildPropertyStrategy {
    fn control(&self, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property(&self, _property: &CurrentProperty, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn property_value(&self, _property: &CurrentProperty, _property_value: &SourceTokenPropertyValue, _ast: &mut AbstractSyntaxTokenStream) {
    }

    fn end_control(&self, _ast: &mut AbstractSyntaxTokenStream) {
    }
}