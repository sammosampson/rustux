
use crate::prelude::*;

#[derive(PartialEq, Eq, Debug)]
enum SourceState {
    Start,
    StartControl,
    InControl(usize),
    EndControl,
    EndNestedControl(usize),
    InProperty(usize),
    InStringPropertyValue(usize),
    InVariablePropertyValue(usize),
    InUSizeNumberPropertyValue(usize),
    InSignedNumberPropertyValue(usize),
    InArrayPropertyValue(usize),
    InCodePropertyValue(usize),
    StartPropertyValue,
    InWhitespace
} 

pub struct SourceTokenizer<'a> {
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    current_parent: Vec<&'a str>,
    state: SourceState
}

impl <'a> Iterator for SourceTokenizer<'a> {
    type Item = SourceTokenResult;
    fn next(&mut self) -> SourceTokenOption {
        loop {
            return match self.characters.next() {
                Some((index, c)) => match self.transition(index, c) {
                    None => continue,
                    result => result
                },
                None => {
                    None
                },
            }
        }
    }
}

impl<'a> SourceTokenizer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: SourceState::Start,
            current_parent: vec![]
        }
    }

    fn transition(&mut self, index: usize, character: char) -> SourceTokenOption {
        match self.state {
            SourceState::Start => {
                self.start_if_possible(index, character)
            },
            SourceState::StartControl => {
                self.start_control_if_possible(index, character)
            },
            SourceState::InControl(start) => {
                self.handle_inside_control(start, index, character)
            },
            SourceState::EndControl => {
                self.end_control_if_possible(index, character)
            },
            SourceState::EndNestedControl(start) => {
                self.end_nested_control_if_possible(start, index, character)
            },
            SourceState::InProperty(start) => {
                self.handle_inside_property(start, index, character)
            },
            SourceState::StartPropertyValue => {
                self.start_property_value_if_possible(index, character)
            },
            SourceState::InStringPropertyValue(start) => {
                self.handle_inside_string_property_value(start, index, character)
            },
            SourceState::InVariablePropertyValue(start) => {
                self.handle_inside_variable_property_value(start, index, character)
            },
            SourceState::InUSizeNumberPropertyValue(start) => {
                self.handle_inside_usize_number_property_value(start, index, character)
            },
            SourceState::InSignedNumberPropertyValue(start) => {
                self.handle_inside_signed_number_property_value(start, index, character)
            },
            SourceState::InArrayPropertyValue(start) => {
                self.handle_inside_array_property_value(start, index, character)
            },
            SourceState::InCodePropertyValue(start) => {
                self.handle_inside_code_property_value(start, index, character)
            },
            SourceState::InWhitespace => {
                self.handle_inside_whitespace(index, character)
            }
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '<' {
            self.state = SourceState::StartControl;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
        Some(Err(SourceTokenError::CouldNotFindStartTag(index)))
    }
    
    fn start_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '/' {
            self.state = SourceState::EndNestedControl(index + 1);
            return None;
        }
        if !character.is_whitespace() {
            self.state = SourceState::InControl(index);
            return None;
        }
        Some(Err(SourceTokenError::CouldNotFindControlName(index)))
    }

    fn produce_control_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        let control_name = self.splice_input(start, index);
        self.current_parent.push(control_name);
        Some(Ok(SourceToken::Control(String::from(control_name))))
    }

    fn handle_inside_control(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character.is_whitespace() {
            self.state = SourceState::InWhitespace;
            return self.produce_control_result(start,index);
        }
        if character == '>' {
            self.state = SourceState::Start;
            return self.produce_control_result(start,index);
        }
        if character == '/' {
            self.state = SourceState::EndControl;
            return self.produce_control_result(start,index);
        }
        None
    }

    fn produce_property_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        let mut input = self.splice_input(start, index);
        let mut token_type = SourceTokenPropertyType::Standard;
        
        if let Some(first_char) = input.chars().nth(0) {
            if first_char == '$' {
                input = self.splice_input(start + 1, index);
                token_type = SourceTokenPropertyType::Variable;
            }
        }
        Some(Ok(SourceToken::Property(token_type, String::from(input))))
    }

    fn handle_inside_property(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character.is_whitespace() {
            self.state = SourceState::InWhitespace;
            return self.produce_property_result(start, index);
        }
        if character == '/' {
            self.state = SourceState::EndControl;
            return self.produce_property_result(start, index);
        }
        if character == '=' {
            self.state = SourceState::StartPropertyValue;
            return self.produce_property_result(start, index);
        }
        if character == '>' {
            self.state = SourceState::Start;
            return self.produce_property_result(start, index);
        }
        None
    }
    
    fn start_property_value_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '"' {
            self.state = SourceState::InStringPropertyValue(index + 1);
            return None;
        }
        if character == '$' {
            self.state = SourceState::InVariablePropertyValue(index + 1);
            return None;
        }
        if character.is_numeric() {
            self.state = SourceState::InUSizeNumberPropertyValue(index);
            return None;
        }
        if character == '-' {
            self.state = SourceState::InSignedNumberPropertyValue(index);
            return None;
        }
        if character == ARRAY_OPENING_CHAR {
            self.state = SourceState::InArrayPropertyValue(index);
            return None;
        }
        if character == CODE_OPENING_CHAR {
            self.state = SourceState::InCodePropertyValue(index);
            return None;
        }
        Some(Err(SourceTokenError::CouldNotFindPropertyStartSymbol(index)))
    }  

    fn produce_string_property_value_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from(self.splice_input(start, index))))))
    }

    fn produce_variable_property_value_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Variable(String::from(self.splice_input(start, index))))))
    }

    fn produce_usize_number_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<usize>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::USize(value)))),
            Err(_) => return self.produce_float_property_value_result(raw_value, index)
        }
    }

    fn produce_signed_number_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<i128>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(value)))),
            Err(_) => return self.produce_float_property_value_result(raw_value, index)
        }
    }

    fn produce_float_property_value_result(&mut self, raw_value: &'a str, index: usize) -> SourceTokenOption {
        match raw_value.parse::<f64>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(value)))),
            Err(_) => return Some(Err(SourceTokenError::CouldNotParseNumberValue(index)))
        }
    }

    fn produce_array_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        let code_content = self.splice_input(start, index);
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Array(tokenize_array(code_content)))))
    }

    fn produce_code_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        let code_content = self.splice_input(start, index);
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Code(tokenize_code(code_content)))))
    }
    
    fn handle_inside_string_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == '"' {
            self.state = SourceState::InWhitespace;
            return self.produce_string_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_variable_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ' ' || character.is_whitespace() {
            self.state = SourceState::InWhitespace;
            return self.produce_variable_property_value_result(start, index);
        }
        if character == '>' {
            self.state = SourceState::Start;
            return self.produce_variable_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_usize_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ' ' || character.is_whitespace() {
            self.state = SourceState::InWhitespace;
            return self.produce_usize_number_property_value_result(start, index);
        }
        if character == '>' {
            self.state = SourceState::Start;
            return self.produce_usize_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ' ' || character.is_whitespace()  {
            self.state = SourceState::InWhitespace;
            return self.produce_signed_number_property_value_result(start, index);
        }        
        if character == '>' {
            self.state = SourceState::Start;
            return self.produce_signed_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_array_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ARRAY_CLOSING_CHAR {
            self.state = SourceState::InWhitespace;
            return self.produce_array_property_value_result(start, index + 1);
        }
        None
    }

    fn handle_inside_code_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == CODE_CLOSING_CHAR {
            self.state = SourceState::InWhitespace;
            return self.produce_code_property_value_result(start, index + 1);
        }
        None
    }

    fn end_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '>' {
            self.state = SourceState::Start;
            match self.current_parent.pop() {
                Some(control_name) => return Some(Ok(SourceToken::EndControl(String::from(control_name)))),
                None => return Some(Err(SourceTokenError::CouldNotFindControlToClose(index)))
            };
        }
        Some(Err(SourceTokenError::CouldNotFindControlCloseSymbol(index)))
    }

    fn end_nested_control_if_possible(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == '>' {
            self.state = SourceState::Start;
            match self.current_parent.pop() {
                Some(control_name) => {
                    let closing_control_name = self.splice_input(start, index);
                    if closing_control_name == control_name {
                        return Some(Ok(SourceToken::EndControl(String::from(control_name))))
                    }
                    return Some(Err(SourceTokenError::ClosingWrongTag(index)))
                },
                None => return Some(Err(SourceTokenError::CouldNotFindControlToClose(index)))
            };
        }
        None
    }

    fn handle_inside_whitespace(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '/' {
            self.state = SourceState::EndControl;
            return None;
        }
        if character == '>' {
            self.state = SourceState::Start;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
                  
        self.state = SourceState::InProperty(index);
        None
    }    
}