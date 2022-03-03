use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct ControlArgument {
    name: String,
    value: AbstractSyntaxPropertyValue
}

impl ControlArgument {
    pub fn parse(name: String, source_property_value: SourceTokenPropertyValue) -> Result<Self, AbstractSyntaxTokenError> {
        let ast_property_value =  match source_property_value {
            SourceTokenPropertyValue::String(value) => AbstractSyntaxPropertyValue::String(value),
            SourceTokenPropertyValue::Int(value) => AbstractSyntaxPropertyValue::Int(value as i32),
            SourceTokenPropertyValue::USize(value) => AbstractSyntaxPropertyValue::USize(value),
            SourceTokenPropertyValue::Float(value) => AbstractSyntaxPropertyValue::Float(value as f32),
            SourceTokenPropertyValue::Code(value) => AbstractSyntaxPropertyValue::Function(Function::parse(&value)?),
            SourceTokenPropertyValue::Variable(value) => AbstractSyntaxPropertyValue::VariablePath(VariablePath::parse(value)?),
            SourceTokenPropertyValue::Array(_) => todo!(),
        };
        Ok(Self { name, value: ast_property_value }) 
    }
}

pub fn create_control_arguments(arguments: Vec<ControlArgument>) -> ControlArguments {
    ControlArguments {
        arguments
    }
}    

#[derive(Debug, Clone, Default)]
pub struct ControlArguments {
    arguments: Vec<ControlArgument>
}

impl From<ControlArguments> for Variables {
    fn from(args: ControlArguments) -> Self {
        let mut variables = Variables::default();
        for arg in args.arguments {
            variables.set(arg.name, arg.value);
        }
        variables
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<AbstractSyntaxPropertyValue> {
        &self.arguments
    }
}

#[derive(Debug, Clone)]
pub struct FloatRange {
    from: f32,
    to: f32
}

impl FloatRange {
    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<FloatRange, AbstractSyntaxTokenError> {
        match collect_array_floats(value, 2) {
            Ok(values) => Ok(FloatRange { from: values[0], to: values[1] }),
            Err(_) => Err(AbstractSyntaxTokenError::RangeValueParseError)
        }
    }
}

impl From<&FloatRange> for RangeInclusive<f32> {
    fn from(from: &FloatRange) -> Self {
        Self::new(from.from, from.to)
    }
}

#[derive(Debug, Clone)]
pub struct USizeRange {
    from: usize,
    to: usize
}

impl USizeRange {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to 
        }
    }

    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<USizeRange, AbstractSyntaxTokenError> {
        match collect_array_usizes(value, 2) {
            Ok(values) => Ok(USizeRange { from: values[0], to: values[1] }),
            Err(_) => Err(AbstractSyntaxTokenError::RangeValueParseError)
        }
    }

    pub fn lower_bound(&self) -> usize {
        self.from
    }

    pub fn upper_bound(&self) -> usize {
        self.to
    }
}

impl From<&USizeRange> for RangeInclusive<usize> {
    fn from(from: &USizeRange) -> Self {
        Self::new(from.from, from.to)
    }
}


#[derive(Debug, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Colour {
    pub fn parse(value: &Vec<ArrayTokenResult>) -> Result<Colour, AbstractSyntaxTokenError> {
        match collect_array_unsigned_shorts(value, 4) {
            Ok(values) => Ok(
                Colour { 
                    r: values[0] as u8,
                    g: values[1] as u8,
                    b: values[2] as u8,
                    a: values[3] as u8 
                }
            ),
            Err(_err) => Err(AbstractSyntaxTokenError::ColourValueParseError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariablePath(String, Option<String>);

impl VariablePath {
    pub fn parse(value: String) -> Result<VariablePath, AbstractSyntaxTokenError> {
        let mut values = value.split('.');
        if let Some(variable_part) = values.next() {
            if let Some(property_part) = values.next() {
                return Ok(Self(variable_part.to_string(), Some(property_part.to_string())))
            }
            return Ok(Self(variable_part.to_string(), None))
        }
        Err(AbstractSyntaxTokenError::VariablePathParseError(value))
    }

    pub fn variable_part(&self) -> &str {
        &self.0
    }
    
    pub fn is_state_variable(&self) -> bool {
        self.variable_part() == "state"
    }

    pub fn property_part(&self) -> &Option<String> {
        &self.1
    }
}