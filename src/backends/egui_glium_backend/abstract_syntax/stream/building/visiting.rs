use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum CurrentProperty {
    None,
    Standard(String),
    Variable(String)
}

pub fn create_ast_token_visitor() -> BuildAbstractSyntaxSourceTokenVisitor {
    BuildAbstractSyntaxSourceTokenVisitor::new()
}

pub struct BuildAbstractSyntaxSourceTokenVisitor {
    pub imports: SourceImports,
    pub ast: AbstractSyntaxTokenStream,
    pub current_property: CurrentProperty,
    strategies: Vec<Box<dyn BuildAbstractSyntaxTokenStreamStrategy + 'static>>
}


impl BuildAbstractSyntaxSourceTokenVisitor {
    fn new() -> Self {
        Self {
            imports: SourceImports::default(),
            ast: AbstractSyntaxTokenStream::default(),
            current_property: CurrentProperty::None,
            strategies: vec!()
        }
    }

    pub fn ast(self) -> AbstractSyntaxTokenStream {
        self.ast
    }   

    fn match_control_name(&mut self, control_name: &str) -> Box<dyn BuildAbstractSyntaxTokenStreamStrategy> {
        match control_name {
            "root" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Root)),
            "control" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Control)),
            "import" => Box::new(ImportBuildAbstractSyntaxTokenStreamStrategy::default()),
            "for" => Box::new(ForBuildAbstractSyntaxTokenStreamStrategy),
            "for-each" => Box::new(ForEachBuildAbstractSyntaxTokenStreamStrategy),
            "let" => Box::new(LetBuildAbstractSyntaxTokenStreamStrategy),
            "central-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::CentralPanel)),
            "top-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::TopPanel)),
            "bottom-panel" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::BottomPanel)),
            "left-side-bar" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::LeftSidebar)),
            "right-side-bar" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::RightSidebar)),
            "scroll-area" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::ScrollArea)),
            "separator" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Separator)),
            "horizontal" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Horizontal)),
            "vertical" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Vertical)),
            "label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Label)),
            "coloured-label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::ColouredLabel)),
            "selectable-label" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::SelectableLabel)),
            "heading" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Heading)),
            "monospace" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Monospace)),
            "code" => Box::new(StandardBuildAbstractSyntaxTokenStreamStrategy(AbstractSyntaxControlType::Code)),
            name => Box::new(ControlReferenceBuildAbstractSyntaxTokenStreamStrategy(name.to_string(), vec!()))
        }
    }
}

impl SourceTokenVisitor for BuildAbstractSyntaxSourceTokenVisitor {
    fn token_error(&mut self, error: SourceTokenError) {
        self.ast.add_error(AbstractSyntaxTokenError::SourceTokenError(error))
    }

    fn control(&mut self, control_name: &str) {
        if let Some(parent_strategy) = self.strategies.last_mut() {
            parent_strategy.child_control(&mut self.ast);
        }
        let strategy = self.match_control_name(control_name);
        strategy.control(&mut self.ast, &self.imports);
        self.strategies.push(strategy);
    }

    fn property(&mut self, property_name: &str) {
        self.current_property = CurrentProperty::Standard(property_name.to_string());
        self.strategies.last_mut().unwrap().property(&self.current_property, &mut self.ast);
    }

    fn variable_property(&mut self, variable_name: &str) {
        self.current_property = CurrentProperty::Variable(variable_name.to_string());
        self.strategies.last_mut().unwrap().property(&self.current_property, &mut self.ast);
    }

    fn property_value(&mut self, property_value: &SourceTokenPropertyValue) {
        self.strategies.last_mut().unwrap().property_value(&self.current_property, property_value, &mut self.ast, &mut self.imports);
    }
    
    fn end_control(&mut self, _control_name: &str) {
        let strategy = self.strategies.pop().unwrap();
        strategy.end_control(&mut self.ast, &self.imports);
    }
}