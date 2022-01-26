use crate::prelude::*;

pub fn create_state_context() -> StateContext {
    StateContext
}

pub struct StateContext;

impl StateContext {
    pub fn run_action_function(function: &ActionFunction) {
    }

    pub fn register_action(&mut self, _action: impl ActionContainer) {
    }
}

pub trait ActionContainer {
    fn path(&self) -> &str;
}



#[derive(Debug, Clone, Default)]
pub struct ActionFunction {
    name: String,
    arguments: Vec<SourceTokenPropertyValue>
}

impl ActionFunction {
    pub fn parse(value: &Vec<CodeTokenResult>) -> Result<ActionFunction, AbstractSyntaxTokenError> {
        let mut function = Self::default();
        for result in value {
            match result {
                Ok(token_value) => match token_value {
                    CodeTokenPropertyValue::StartFunction(function_name) => function.name = function_name.clone(),
                    CodeTokenPropertyValue::PropertyValue(property_value) => function.arguments.push(property_value.clone()),
                    CodeTokenPropertyValue::EndFunction => {},
                },
                Err(err) => return Err(AbstractSyntaxTokenError::CodeTokenError(err.clone())),
            }
        }

        Ok(function)
    }
}
