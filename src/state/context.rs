use crate::prelude::*;

pub fn create_data_context() -> DataContext {
    DataContext::default()
}

#[derive(Debug)]
pub enum DataContextError {
    ActionRunError(ContainerRunError),
    DataArrayItemDoesNotExist(VariablePath),
    DataArrayDoesNotExist,
    VariableDoesNotExist,
    ContainerNotFound
}


impl From<ContainerRunError> for DataContextError {
    fn from(from: ContainerRunError) -> Self {
        Self::ActionRunError(from)
    }
}

#[derive(Default)]
pub struct DataContext {
    actions: RegisteredActions,
    selectors: RegisteredSelectors,
    data_arrays: DataArrays,
    scopes: DataScopes
}

impl DataContext {
    pub fn scopes_mut(&mut self) -> &mut DataScopes {
        &mut self.scopes
    }

    pub fn current_scope_mut(&mut self) -> &mut DataScope {
        self.scopes.current_mut()
    }

    pub fn data_arrays(&self) -> &DataArrays {
        &self.data_arrays
    }

    pub fn data_arrays_mut(&mut self) -> &mut DataArrays {
        &mut self.data_arrays
    }

    pub fn actions_mut(&mut self) -> &mut RegisteredActions {
        &mut self.actions
    }

    pub fn selectors_mut(&mut self) -> &mut RegisteredSelectors {
        &mut self.selectors
    }

    pub fn run_action_function(&mut self, function: &Function) -> Result<(), DataContextError> {
        self.actions.run_action_function(self.scopes.current_mut(), function)
    }

    pub fn run_selector_function(&mut self, function: &Function) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        self.selectors.run_selector_function(&mut self.data_arrays, self.scopes.current_mut(), function)
    }

    pub fn replace_variable_data_in_property(&mut self, property: AbstractSyntaxProperty) -> Result<AbstractSyntaxProperty, DataContextError> {
        self.scopes
            .current_mut()
            .variables_mut()
            .replace_variable_data_in_property(&self.data_arrays, property)
    }

    pub fn replace_variable_data_in_function(&mut self, function: &Function) -> Result<Function, DataContextError> {
        self.scopes
            .current_mut()
            .variables_mut()
            .replace_variable_data_in_function(&self.data_arrays, function)
    }
}
