use crate::prelude::*;


pub fn create_data_context() -> DataContext {
    DataContext::default()
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

    pub fn get_local<T:std::any::Any + Default>(&mut self, id: usize) -> &T {
        if !self.items.contains_key(&id) {
            self.set(id, T::default());
        }
        self.get(id).unwrap().downcast_ref::<T>().unwrap()
    }

    fn get(&self, id: usize) -> Option<&Box<dyn std::any::Any>> {
        self.items.get(&id)
    }
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
        if let Some(container) = self.actions.get_action_container(&function.name) {
            container.run(&mut self.state, &function.arguments)?;
            return Ok(());
        }
        Err(DataContextError::ContainerNotFound)
    }

    pub fn run_selector_function(&mut self, function: &Function) -> Result<AbstractSyntaxPropertyValue, DataContextError> {
        if let Some(container) = self.selectors.get_selector_container(&function.name) {
            return Ok(container.run(&mut self.state, &function.arguments)?);
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
        for argument in &function.arguments {
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

#[derive(Debug)]
pub enum ContainerRunError {
    ContainerNotFound,
    IncorrectAmountOfArgumentsPassed,
    FirstArgumentNotStateVariable,
    PropertyValueError(AbstractSyntaxPropertyValueError)
}

impl From<AbstractSyntaxPropertyValueError> for ContainerRunError {
    fn from(from: AbstractSyntaxPropertyValueError) -> Self {
        Self::PropertyValueError(from)
    }
}

pub trait SelectorContainer {
    fn function_name(&self) -> &str;
    fn run(&self, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError>;
}

#[derive(Default)]
pub struct RegisteredSelectors {
    selectors: HashMap<String, Box<dyn SelectorContainer>>
}

impl RegisteredSelectors {
    pub fn register_selector(&mut self, selector: impl SelectorContainer + 'static) {
        self.selectors.insert(selector.function_name().to_string(), Box::new(selector));
    }

    pub fn get_selector_container(&self, function_name: &str) -> Option<&Box<dyn SelectorContainer>> {
        self.selectors.get(function_name)
    }
}

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


#[derive(Debug, Clone)]
pub enum AbstractSyntaxPropertyValue {
    String(String),
    Bool(bool),
    TextStyle(TextStyle),
    Float(f32),
    FloatRange(FloatRange),
    VerticalSize(VerticalSize), 
    Colour(Colour), 
    Function(Function),
    USize(usize),
    USizeRangeVariable(String, USizeRange),
    FunctionVariable(String, Function),
    Variable(String)
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxPropertyValueError {
    ValueNotExpected(AbstractSyntaxPropertyValue)
}

impl AbstractSyntaxPropertyValue {
    pub fn is_state_variable(&self) -> bool {
        if let Self::Variable(value) = self {
            return value == "state";
        }
        false
    }

    pub fn get_string_value(&self) -> Result<String, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::String(value) = self.clone() {
            return Ok(value);
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_float_value(&self) -> Result<f32, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::Float(value) = self {
            return Ok(*value);
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_float_range_value(&self) -> Result<RangeInclusive::<f32>, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::FloatRange(value) = &self {
            return Ok(value.into());
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_usize_range_variable_value(&self) -> Result<(String, USizeRange), AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::USizeRangeVariable(variable, range) = self.clone() {
            return Ok((variable, range));
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_usize_value(&self) -> Result<usize, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::USize(value) = self.clone() {
            return Ok(value);
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_bool_value(&self) -> Result<bool, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::Bool(value) = self {
            return Ok(*value);
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }
    
    pub fn get_colour_value(&self) -> Result<egui::Color32, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::Colour(value) = &self {
            return Ok(value.into());
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }
    
    pub fn get_text_style_value(&self) -> Result<egui::TextStyle, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::TextStyle(value) = &self {
            return Ok(value.into());
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }
    
    pub fn get_vertical_size_value(&self) -> Result<VerticalSize, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::VerticalSize(value) = self.clone() {
            return Ok(value);
        }
        panic!()
    }
    
    pub fn get_function_value(&self) -> Result<Function, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::Function(value) = self.clone() {
            return Ok(value);
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }

    pub fn get_function_variable_value(&self) -> Result<(String, Function), AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::FunctionVariable(variable, function) = self.clone() {
            return Ok((variable, function));
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
    }
}

impl From<&SourceTokenPropertyValue> for AbstractSyntaxPropertyValue {
    fn from(from: &SourceTokenPropertyValue) -> Self {
        match from {
            SourceTokenPropertyValue::String(value) => Self::String(value.clone()),
            SourceTokenPropertyValue::Float(value) => Self::Float(*value as f32),
            SourceTokenPropertyValue::Variable(value) => Self::Variable(value.clone()),
            _  => panic!(),
        }
    }
}
        
#[derive(Debug, Clone, Default)]
pub struct Function {
    name: String,
    arguments: Vec<AbstractSyntaxPropertyValue>
}

impl Function {
    pub fn parse(value: &Vec<CodeTokenResult>) -> Result<Function, AbstractSyntaxTokenError> {
        let mut function = Self::default();
        for result in value {
            match result {
                Ok(token_value) => match token_value {
                    CodeTokenPropertyValue::StartFunction(function_name) => function.name = function_name.clone(),
                    CodeTokenPropertyValue::PropertyValue(property_value) => function.arguments.push(property_value.into()),
                    CodeTokenPropertyValue::EndFunction => {},
                },
                Err(err) => return Err(AbstractSyntaxTokenError::CodeTokenError(err.clone())),
            }
        }

        Ok(function)
    }

    pub fn set_arguments(&self, arguments: Vec<AbstractSyntaxPropertyValue>) -> Self {
        Self {
            name: self.name.clone(),
            arguments
        }
    }
}