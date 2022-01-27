use crate::prelude::*;

pub fn create_state_context() -> StateContext {
    StateContext::default()
}

#[derive(Default)]
pub struct StateContext {
    actions: HashMap<String, Box<dyn ActionContainer>>
}

impl StateContext {
    pub fn run_action_function(&mut self, function: &ActionFunction) {
        if let Some(container) = self.get_action_container(&function.name) {
            container.run(&function.arguments);
        }
    }

    pub fn register_action(&mut self, action: impl ActionContainer + 'static) {
        self.actions.insert(action.function_name().to_string(), Box::new(action));
    }

    pub(crate) fn get_action_container(&self, function_name: &str) -> Option<&Box<dyn ActionContainer>> {
        self.actions.get(function_name)
    }
}

pub trait ActionContainer {
    fn function_name(&self) -> &str;
    fn run(&self, arguments: &Vec<SourceTokenPropertyValue>);
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
