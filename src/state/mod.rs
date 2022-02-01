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
        let processed_state = if let Some(item) = self.get(id) {
            processor(item.downcast_ref::<T>().unwrap())
        } else {
            processor(&T::default())
        };

        self.set(id, processed_state)
    }

    pub fn set<T:std::any::Any + Default>(&mut self, id: usize, to_set: T) {
        self.items.insert(id, Box::new(to_set));
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
    pub fn run_action_function(&mut self, function: &Function) {
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
pub struct Function {
    name: String,
    arguments: Vec<SourceTokenPropertyValue>
}

impl Function {
    pub fn parse(value: &Vec<CodeTokenResult>) -> Result<Function, AbstractSyntaxTokenError> {
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


#[derive(Debug, Clone)]
pub enum VariablePropertyType {
    Usize(usize)
}


#[derive(Default)]
pub struct DataContext(HashMap<String, VariablePropertyType>);

impl DataContext {
    pub fn set_variable(&mut self, variable: String, variable_value: VariablePropertyType) {
        self.0.insert(variable, variable_value);
    }
}