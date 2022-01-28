use crate::prelude::*;

use std::iter::Enumerate;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug)]
enum CodeState {
    Start,
    StartFunction,
    InFunction(usize),
    StartValue,
    InSignedNumberValue(usize),
    InUnsignedNumberValue(usize),
    InStringValue(usize),
    EndValue,
    EndFunction,
    End,
    InWhitespace
}


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum CodeTokenPropertyValue {
    StartFunction(String),
    PropertyValue(SourceTokenPropertyValue),
    EndFunction
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Clone)]
pub enum CodeTokenError {
    NoOpeningBrace(usize),
    NoClosingBrace(usize),
    NoOpeningFunctionParenthesis(usize),
    NoClosingFunctionParenthesis(usize),
    ParseNumberError(usize, String),
}

pub type CodeTokenResult = Result<CodeTokenPropertyValue, CodeTokenError>;
pub type CodeTokenOption = Option<CodeTokenResult>;

pub fn tokenize_code(from: &str) -> Vec::<CodeTokenResult> {
    CodeTokenizer::from_string(from).collect()
}

pub struct CodeTokenizer<'a>{
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    state: CodeState
}

impl<'a> CodeTokenizer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: CodeState::Start
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char) -> CodeTokenOption {
        if character == CODE_OPENING_CHAR {
            self.state = CodeState::StartFunction;
            return None;
        }
        Some(Err(CodeTokenError::NoOpeningBrace(index)))
    }

    fn end_if_possible(&mut self, index: usize, character: char) -> CodeTokenOption {
        if character == CODE_CLOSING_CHAR {
            self.state = CodeState::End;
            return Some(Ok(CodeTokenPropertyValue::EndFunction));
        }
        Some(Err(CodeTokenError::NoClosingBrace(index)))
    }

    fn start_function_if_possible(&mut self, index: usize, character: char) -> CodeTokenOption {
        if character == FUNCTION_OPENING_BRACE {
            self.state = CodeState::StartValue;
        } else {
            self.state = CodeState::InFunction(index);
        }
        None
    }

    fn start_value_if_possible(&mut self, index: usize, character: char) -> CodeTokenOption {
        if character == FUNCTION_CLOSING_BRACE {
            self.state = CodeState::EndFunction;
            return None;
        }
        if character.is_numeric() {
            self.state = CodeState::InUnsignedNumberValue(index);
            return None;
        }
        if character == '-' {
            self.state = CodeState::InSignedNumberValue(index);
            return None;
        }        
        if character == '"' {
            self.state = CodeState::InStringValue(index + 1);
            return None;
        }
        if character == ' ' {
            self.state = CodeState::InWhitespace;
            return None;
        }
        Some(Err(CodeTokenError::NoClosingFunctionParenthesis(index)))
    }
    
    fn produce_function_name_result(&mut self, start: usize, index: usize) -> CodeTokenOption {
        let value = self.splice_input(start, index);
        return Some(Ok(CodeTokenPropertyValue::StartFunction(value.to_string())));
    }

    fn produce_signed_number_value_result(&mut self, start: usize, index: usize) -> CodeTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<i128>() {
            Ok(value) => return Some(Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::Int(value)))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_unsigned_number_value_result(&mut self, start: usize, index: usize) -> CodeTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<u128>() {
            Ok(value) => return Some(Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::UnsignedInt(value)))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_float_value_result(&mut self, raw_value: &'a str, index: usize) -> CodeTokenOption {
        match raw_value.parse::<f64>() {
            Ok(value) => return Some(Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::Float(value)))),
            Err(_) => return Some(Err(CodeTokenError::ParseNumberError(index, raw_value.to_string())))
        }
    }

    fn produce_string_value_result(&mut self, start: usize, index: usize) -> CodeTokenOption {
        let value = self.splice_input(start, index);
        return Some(Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::String(value.to_string()))));
    }

    
    fn handle_inside_function(&mut self, start: usize, index: usize, character: char) -> CodeTokenOption {
        if character == FUNCTION_OPENING_BRACE {
            self.state = CodeState::StartValue;
            return self.produce_function_name_result(start, index);
        }
        None
    }


    fn handle_inside_unsigned_number_value(&mut self, start: usize, index: usize, character: char) -> CodeTokenOption {
        if character == FUNCTION_CLOSING_BRACE {
            self.state = CodeState::EndFunction;
            return self.produce_unsigned_number_value_result(start, index);
        }
        if character == ',' {
            self.state = CodeState::EndValue;
            return self.produce_unsigned_number_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_value(&mut self, start: usize, index: usize, character: char) -> CodeTokenOption {
        if character == FUNCTION_CLOSING_BRACE {
            self.state = CodeState::EndFunction;
            return self.produce_signed_number_value_result(start, index);
        }
        if character == ',' {
            self.state = CodeState::EndValue;
            return self.produce_signed_number_value_result(start, index);
        }
        None
    }

    fn handle_inside_string_value(&mut self, start: usize, index: usize, character: char) -> CodeTokenOption {
        if character == '"' {
            self.state = CodeState::EndValue;
            return self.produce_string_value_result(start, index);
        }
        None
    }

    fn transition(&mut self, index: usize, character: char) -> CodeTokenOption {
        match self.state {
            CodeState::Start => {
                self.start_if_possible(index, character)
            },            
            CodeState::StartFunction => {
                self.start_function_if_possible(index, character)
            },
            CodeState::InFunction(start) => {
                self.handle_inside_function(start, index, character)
            },
            CodeState::StartValue => {
                self.start_value_if_possible(index, character)
            },
            CodeState::InSignedNumberValue(start) => {
                self.handle_inside_signed_number_value(start, index, character)
            },
            CodeState::InUnsignedNumberValue(start) => {
                self.handle_inside_unsigned_number_value(start, index, character)
            },
            CodeState::InStringValue(start) => {
                self.handle_inside_string_value(start, index, character)
            },
            CodeState::EndValue => {
                self.start_value_if_possible(index, character)
            },
            CodeState::InWhitespace => {
                self.start_value_if_possible(index, character)
            },
            CodeState::EndFunction => {
                self.end_if_possible(index, character)
            },
            CodeState::End => {
                None
            }
        }
    }
}

impl <'a> Iterator for CodeTokenizer<'a> {
    type Item = CodeTokenResult;
    fn next(&mut self) -> CodeTokenOption {
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