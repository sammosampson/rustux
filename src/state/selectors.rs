use crate::prelude::*;

pub trait SelectorContainer {
    fn function_name(&self) -> &str;
    fn run(
        &self,
        data_arrays: &mut DataArrays,
        state: &mut State,
        arguments: &Vec<AbstractSyntaxPropertyValue>
    ) -> Result<AbstractSyntaxPropertyValue, ContainerRunError>;
}

#[derive(Default)]
pub struct RegisteredSelectors {
    selectors: HashMap<String, Box<dyn SelectorContainer>>
}

impl RegisteredSelectors {
    pub fn register_selector(&mut self, selector: impl SelectorContainer + 'static) {
        self.selectors.insert(selector.function_name().to_string(), Box::new(selector));
    }
    
    pub fn run_selector_function(
        &mut self,
        data_arrays: &mut DataArrays,
        scope: &mut DataScope,
        function: &Function
    ) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(container) = self.get_selector_container(function.name()) {
            return Ok(container.run(data_arrays, scope.state_mut(), &function.arguments())?);
        }
        Err(DataContextError::ContainerNotFound)
    }
    
    fn get_selector_container(&self, function_name: &str) -> Option<&Box<dyn SelectorContainer>> {
        self.selectors.get(function_name)
    }
}
