use crate::prelude::*;

pub fn create_state_context() -> StateContext {
    StateContext::default()
}

#[derive(Default)]
pub struct RegisteredActions {
    actions: HashMap<String, Box<dyn ActionContainer>>
}

impl RegisteredActions {
    pub fn register_action(&mut self, action: impl ActionContainer + 'static) {
        self.actions.insert(action.function_name().to_string(), Box::new(action));
    }

    pub(crate) fn get_action_container(&self, function_name: &str) -> Option<&Box<dyn ActionContainer>> {
        self.actions.get(function_name)
    }
}

#[derive(Default)]
pub struct State {
    items: HashMap<usize, Box<dyn std::any::Any>>
}

impl State {
    pub fn process<T:std::any::Any + Default>(&mut self, id: usize, processor: Box<dyn FnOnce(&T) -> T>) {
        if let Some(item) = self.get(id) {
            self.set(id, processor(item.downcast_ref::<T>().unwrap()));
        } else {
            self.set(id, processor(&T::default()));
        }
    }

    fn get_or_default<T:std::any::Any + Default>(&mut self, id: usize) -> &T {
        if !self.contains(id) {
            self.set(id, T::default());
        }

        let item = self.get(id).unwrap();
        item.downcast_ref::<T>().unwrap()
    }

    pub fn set<T:std::any::Any + Default>(&mut self, id: usize, to_set: T) {
        self.items.insert(id, Box::new(to_set));
    }

    fn contains(&self, id: usize) -> bool {
        self.items.contains_key(&id)
    }

    fn get(&self, id: usize) -> Option<&Box<dyn std::any::Any>> {
        self.items.get(&id)
    }
}

#[derive(Default)]
pub struct StateContext {
    actions: RegisteredActions,
    state: State
}

impl StateContext {
    pub fn run_action_function(&mut self, function: &ActionFunction) {
        let container = self.actions.get_action_container(&function.name).unwrap();
        container.run(&mut self.state, &function.arguments);
    }

    pub fn actions_mut(&mut self) -> &mut RegisteredActions {
        &mut self.actions
    }
}

pub trait ActionContainer {
    fn function_name(&self) -> &str;
    fn run(&self, state: &mut State, arguments: &Vec<SourceTokenPropertyValue>);
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
