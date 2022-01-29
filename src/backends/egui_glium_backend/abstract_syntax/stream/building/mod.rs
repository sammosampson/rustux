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
    current_property_strategy: Box<dyn BuildAbstractSyntaxTokenStreamStrategy + 'static>
}

impl Default for BuildAbstractSyntaxSourceTokenVisitor {
    fn default() -> Self {
        Self {
            ast: AbstractSyntaxTokenStream::default(),
            current_property: CurrentProperty::None,
            current_property_strategy: Box::new(EmptyBuildAbstractSyntaxTokenStreamStrategy)
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

fn match_control_name(control_name: &str) -> Box<dyn BuildAbstractSyntaxTokenStreamStrategy> {
    match control_name {
        "root" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Root)),
        "for" => Box::new(ForBuildAbstractSyntaxTokenStreamStrategy),
        "let" => Box::new(LetBuildAbstractSyntaxTokenStreamStrategy),
        "central-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::CentralPanel)),
        "top-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::TopPanel)),
        "bottom-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::BottomPanel)),
        "left-side-bar" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::LeftSidebar)),
        "right-side-bar" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::RightSidebar)),
        "scroll-area" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::ScrollArea)),
        "separator" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Separator)),
        "horizontal" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Horizontal)),
        "vertical" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Vertical)),
        "label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Label)),
        "coloured-label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::ColouredLabel)),
        "selectable-label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::SelectableLabel)),
        "heading" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Heading)),
        "monospace" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Monospace)),
        "code" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Code)),
        _ => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxTokenType::Unknown))
    }
}
