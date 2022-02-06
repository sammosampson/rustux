
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum AbstractSyntaxPropertyValue {
    String(String),
    Bool(bool),
    Float(f32),
    FloatRange(FloatRange),
    Colour(Colour), 
    Function(Function),
    USize(usize),
    USizeRangeVariable(String, USizeRange),
    FunctionVariable(String, Function),
    VariablePath(VariablePath),
    DataArray(DataArrayId, usize)
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxPropertyValueError {
    ValueNotExpected(AbstractSyntaxPropertyValue)
}

impl AbstractSyntaxPropertyValue {
    pub fn is_state_variable(&self) -> bool {
        if let Self::VariablePath(value) = self {
            return value.is_state_variable()
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
    
    pub fn get_colour_value(&self) -> Result<Colour, AbstractSyntaxPropertyValueError> {
        if let AbstractSyntaxPropertyValue::Colour(value) = &self {
            return Ok(value.clone());
        }
        Err(AbstractSyntaxPropertyValueError::ValueNotExpected(self.clone()))
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
            SourceTokenPropertyValue::Variable(value) => Self::VariablePath(VariablePath::parse(value.clone()).unwrap()),
            _  => panic!(),
        }
    }
}