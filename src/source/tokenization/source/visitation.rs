use crate::prelude::*;

pub trait SourceTokenVisitor {
    fn token_error(&mut self, error: SourceTokenError);
    fn control(&mut self, control_name: &str);
    fn property(&mut self, property_name: &str);
    fn variable_property(&mut self, variable_name: &str);
    fn property_value(&mut self, property_value: &SourceTokenPropertyValue);
    fn end_control(&mut self, control_name: &str);
}

pub struct SourceTokenVisitationNavigator<'a> {
    tokenizer: SourceTokenizer<'a>
}

impl<'a> SourceTokenVisitationNavigator<'a> {
    pub fn from_source(tokenizer: SourceTokenizer<'a>) -> SourceTokenVisitationNavigator {
        Self {
            tokenizer
        }
    }

    pub fn accept(self, visitor: &mut impl SourceTokenVisitor) {
        for token_result in self.tokenizer {
            match token_result {
                Ok(token) => match token {
                    SourceToken::Control(control_name) => visitor.control(&control_name),
                    SourceToken::Property(property_type, property_name) => match property_type {
                        SourceTokenPropertyType::Standard => visitor.property(&property_name),
                        SourceTokenPropertyType::Variable =>  visitor.variable_property(&property_name),
                    }
                    SourceToken::PropertyValue(property_value) => visitor.property_value(&property_value),
                    SourceToken::EndControl(control_name) => visitor.end_control(&control_name),
                },
                Err(error) => visitor.token_error(error),
            }
        }
    }
}