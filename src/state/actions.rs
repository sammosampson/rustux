use crate::prelude::*;

pub trait ActionContainer {
    fn function_name(&self) -> &str;
    fn run(&self, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<(), ContainerRunError>;
}

#[derive(Default)]
pub struct RegisteredActions {
    actions: HashMap<String, Box<dyn ActionContainer>>
}

impl RegisteredActions {
    pub fn register_action(&mut self, action: impl ActionContainer + 'static) {
        self.actions.insert(action.function_name().to_string(), Box::new(action));
    }

    pub fn get_action_container(&self, function_name: &str) -> Option<&Box<dyn ActionContainer>> {
        self.actions.get(function_name)
    }
}
