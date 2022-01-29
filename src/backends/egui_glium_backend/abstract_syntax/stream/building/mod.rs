mod strategies;
pub use strategies::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum CurrentProperty {
    None,
    Standard(String),
    Variable(String)
}

pub struct BuildAbstractSyntaxSourceTokenVisitor {
    ast: AbstractSyntaxTokenStream,
    current_property: CurrentProperty,
    current_property_strategy: Box<dyn BuildPropertyStrategy + 'static>
}

impl Default for BuildAbstractSyntaxSourceTokenVisitor {
    fn default() -> Self {
        Self {
            ast: AbstractSyntaxTokenStream::default(),
            current_property: CurrentProperty::None,
            current_property_strategy: Box::new(EmptyBuildPropertyStrategy)
        }
    }
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
        self.current_property_strategy = match_control_name(control_name);
        self.current_property_strategy.control(&mut self.ast);
    }

    fn property(&mut self, property_name: &str) {
        self.current_property = CurrentProperty::Standard(property_name.to_string());
        self.current_property_strategy.property(&self.current_property, &mut self.ast);
    }

    fn variable_property(&mut self, variable_name: &str) {
        self.current_property = CurrentProperty::Variable(variable_name.to_string());
        self.current_property_strategy.property(&self.current_property, &mut self.ast);
    }

    fn property_value(&mut self, property_value: &SourceTokenPropertyValue) {
        self.current_property_strategy.property_value(&self.current_property, property_value, &mut self.ast);
    }
    
    fn end_control(&mut self, _control_name: &str) {
        self.current_property_strategy.end_control(&mut self.ast);
    }
}

fn match_control_name(control_name: &str) -> Box<dyn BuildPropertyStrategy> {
    match control_name {
        "root" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Root)),
        "for" => Box::new(ForBuildPropertyStrategy),
        "let" => Box::new(LetBuildPropertyStrategy),
        "central-panel" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::CentralPanel)),
        "top-panel" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::TopPanel)),
        "bottom-panel" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::BottomPanel)),
        "left-side-bar" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::LeftSidebar)),
        "right-side-bar" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::RightSidebar)),
        "scroll-area" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::ScrollArea)),
        "separator" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Separator)),
        "horizontal" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Horizontal)),
        "vertical" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Vertical)),
        "label" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Label)),
        "coloured-label" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::ColouredLabel)),
        "selectable-label" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::SelectableLabel)),
        "heading" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Heading)),
        "monospace" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Monospace)),
        "code" => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Code)),
        _ => Box::new(StandardBuildPropertyStrategy(AbstractSyntaxTokenType::Unknown))
    }
}
