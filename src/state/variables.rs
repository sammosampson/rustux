
use crate::prelude::*;

#[derive(Default)]
pub struct Variables {
    inner: HashMap<String, AbstractSyntaxPropertyValue>
}

impl Variables {
    pub fn set(&mut self, variable: String, variable_value: AbstractSyntaxPropertyValue) {
        self.inner.insert(variable, variable_value);
    }

    pub fn remove(&mut self, variable: &str) {
        self.inner.remove(variable);
    }

    pub fn get(&self, variable: &str) -> Option<&AbstractSyntaxPropertyValue> {
        self.inner.get(variable)
    }

    pub fn replace_variable_data_in_property(
        &mut self,
        data_arrays: &DataArrays, 
        property: AbstractSyntaxProperty
    ) -> Result<AbstractSyntaxProperty, DataContextError> {
        match property.value() {
            AbstractSyntaxPropertyValue::VariablePath(_) |
            AbstractSyntaxPropertyValue::Function(_) => {
                Ok(property.set_value(self.replace_variable_data_in_value(data_arrays, property.value())?))
            },
            _ => return Ok(property),
        }
    }

    fn replace_variable_data_in_value(
        &mut self,
        data_arrays: &DataArrays,
        property_value: &AbstractSyntaxPropertyValue
    ) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        match property_value {
            AbstractSyntaxPropertyValue::Function(function) =>
                Ok(AbstractSyntaxPropertyValue::Function(self.replace_variable_data_in_function(data_arrays, function)?)),
            AbstractSyntaxPropertyValue::VariablePath(variable) => 
                Ok(self.get_variable_value(data_arrays, variable)?),
            _ =>
                Ok(property_value.clone())
        }
    }

    pub fn replace_variable_data_in_function(
        &mut self,        
        data_arrays: &DataArrays,
        function: &Function
    ) -> Result<Function, DataContextError> {
        let mut resolved_arguments = vec!();
        for argument in function.arguments() {
            if argument.is_state_variable() {
                resolved_arguments.push(argument.clone());
            }  else {          
                resolved_arguments.push(self.replace_variable_data_in_value(data_arrays, argument)?);
            }
        }
        return Ok(function.set_arguments(resolved_arguments));
    }

    fn get_variable_value(&self, data_arrays: &DataArrays, variable: &VariablePath) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(variable_value) = self.get(variable.variable_part()) {
            match variable_value {
                AbstractSyntaxPropertyValue::DataArray(array_id, position) => return data_arrays.get_array_item_value(*array_id, *position, variable),
                _ => return Ok(variable_value.clone())
            }
        } 
        return Err(DataContextError::VariableDoesNotExist);
    }
}