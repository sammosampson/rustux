use crate::prelude::*;

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

#[derive(Debug, Clone, Copy)]
pub enum VerticalSize {
    Auto,
    MaxHeight(f32)
}

#[derive(Debug, Clone)]
pub enum TextStyle {
    Small,
    Body,
    Button,
    Heading,
    Monospace,
}

impl TextStyle {
    pub fn parse(value: &str) -> Result<TextStyle, AbstractSyntaxTokenError> {
        match value {
            "small" => Ok(TextStyle::Small),
            "body" => Ok(TextStyle::Body),
            "button" => Ok(TextStyle::Button),
            "heading" => Ok(TextStyle::Heading),
            "monospace" => Ok(TextStyle::Monospace),
            _ => Err(AbstractSyntaxTokenError::TextStyleValueParseError(value.to_string()))
        }
    }
}