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
    state: State,
    variables: HashMap<String, AbstractSyntaxPropertyValue> 
}

impl DataContext {
    pub fn run_action_function(&mut self, function: &Function) -> Result<(), DataContextError> {
        if let Some(container) = self.actions.get_action_container(function.name()) {
            container.run(&mut self.state, &function.arguments())?;
            return Ok(());
        }
        Err(DataContextError::ContainerNotFound)
    }

    pub fn run_selector_function(&mut self, function: &Function) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(container) = self.selectors.get_selector_container(function.name()) {
            return Ok(container.run(&mut self.data_arrays, &mut self.state, &function.arguments())?);
        }
        Err(DataContextError::ContainerNotFound)
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

    pub fn set_variable(&mut self, variable: String, variable_value: AbstractSyntaxPropertyValue) {
        self.variables.insert(variable, variable_value);
    }

    pub fn remove_variable(&mut self, variable: &str) {
        self.variables.remove(variable);
    }

    pub fn replace_variable_data_in_property(
        &mut self,
        property: AbstractSyntaxProperty
    ) -> Result<AbstractSyntaxProperty, DataContextError> {
        match property.value() {
            AbstractSyntaxPropertyValue::VariablePath(_) |
            AbstractSyntaxPropertyValue::Function(_) => {
                Ok(property.set_value(self.replace_variable_data_in_value(property.value())?))
            },
            _ => return Ok(property),
        }
    }

    fn replace_variable_data_in_value(
        &mut self,
        property_value: &AbstractSyntaxPropertyValue
    ) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        match property_value {
            AbstractSyntaxPropertyValue::Function(function) =>
                Ok(AbstractSyntaxPropertyValue::Function(self.replace_variable_data_in_function(function)?)),
            AbstractSyntaxPropertyValue::VariablePath(variable) => 
                Ok(self.get_variable_value(variable)?),
            _ =>
                Ok(property_value.clone())
        }
    }

    pub fn replace_variable_data_in_function(&mut self, function: &Function) -> Result<Function, DataContextError> {
        let mut resolved_arguments = vec!();
        for argument in function.arguments() {
            if argument.is_state_variable() {
                resolved_arguments.push(argument.clone());
            }  else {          
                resolved_arguments.push(self.replace_variable_data_in_value(argument)?);
            }
        }
        return Ok(function.set_arguments(resolved_arguments));
    }

    fn get_variable_value(&self, variable: &VariablePath) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(variable_value) = self.variables.get(variable.variable_part()) {
            match variable_value {
                AbstractSyntaxPropertyValue::DataArray(array_id, position) => return self.get_array_item_value(*array_id, *position, variable),
                _ => return Ok(variable_value.clone())
            }
        } 
        return Err(DataContextError::VariableDoesNotExist);
    }

    fn get_array_item_value(&self, array_id: DataArrayId, position: usize, variable: &VariablePath) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(array) = self.data_arrays().get(array_id) {
            if let Some(value) = array.get_array_item_value(position, variable) {
                return Ok(value);
            } else {
                return Err(DataContextError::DataArrayItemDoesNotExist(variable.clone()))        
            }
        }
        
        Err(DataContextError::DataArrayDoesNotExist)
    }
}