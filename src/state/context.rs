use crate::prelude::*;

pub fn create_data_context() -> DataContext {
    DataContext::default()
}

#[derive(Debug)]
pub enum DataContextError {
    ActionRunError(ContainerRunError),
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
            return Ok(container.run(&mut self.state, &function.arguments())?);
        }
        Err(DataContextError::ContainerNotFound)
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

    pub fn replace_variable_data_in_property(
        &mut self,
        property: AbstractSyntaxProperty
    ) -> Result<AbstractSyntaxProperty, DataContextError> {
        match property.value() {
            AbstractSyntaxPropertyValue::Variable(_) |
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
            AbstractSyntaxPropertyValue::Variable(variable) => 
                Ok(self.get_variable(&variable)?),
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

    fn get_variable(&self, variable: &str) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(variable_value) = self.variables.get(variable) {
            return Ok(variable_value.clone());
        } 
        return Err(DataContextError::VariableDoesNotExist);
    }
}