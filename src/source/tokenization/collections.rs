use crate::prelude::*;

use std::iter::Enumerate;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug)]
enum ArrayState {
    Start,
    StartValue,
    EndArray,
    InSignedNumberValue(usize),
    InUSizeNumberValue(usize),
    InStringValue(usize),
    EndValue,
    InWhitespace
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Clone)]
pub enum ArrayTokenError {
    NoOpeningParenthesis(usize),
    NoClosingParenthesis(usize),
    ParseNumberError(usize, String),
}

pub type ArrayTokenResult = Result<SourceTokenPropertyValue, ArrayTokenError>;
pub type ArrayTokenOption = Option<ArrayTokenResult>;

pub fn tokenize_array(from: &str) -> Vec::<ArrayTokenResult> {
    ArrayTokenizer::from_string(from).collect()
}

pub struct ArrayTokenizer<'a>{
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    state: ArrayState
} 

impl<'a> ArrayTokenizer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: ArrayState::Start
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char) -> ArrayTokenOption {
        if character == ARRAY_OPENING_CHAR {
            self.state = ArrayState::StartValue;
            return None;
        }
        Some(Err(ArrayTokenError::NoOpeningParenthesis(index)))
    }

    fn start_value_if_possible(&mut self, index: usize, character: char) -> ArrayTokenOption {
        if character == ARRAY_CLOSING_CHAR {
            self.state = ArrayState::EndArray;
            return None;
        }
        if character.is_numeric() {
            self.state = ArrayState::InUSizeNumberValue(index);
            return None;
        }
        if character == '-' {
            self.state = ArrayState::InSignedNumberValue(index);
            return None;
        }        
        if character == '"' {
            self.state = ArrayState::InStringValue(index + 1);
            return None;
        }
        if character == ' ' {
            self.state = ArrayState::InWhitespace;
            return None;
        }
        Some(Err(ArrayTokenError::NoClosingParenthesis(index)))
    }
    
    fn produce_signed_number_value_result(&mut self, start: usize, index: usize) -> ArrayTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<i128>() {
            Ok(value) => return Some(Ok(SourceTokenPropertyValue::Int(value))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_usize_value_result(&mut self, start: usize, index: usize) -> ArrayTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<usize>() {
            Ok(value) => return Some(Ok(SourceTokenPropertyValue::USize(value))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_float_value_result(&mut self, raw_value: &'a str, index: usize) -> ArrayTokenOption {
        match raw_value.parse::<f64>() {
            Ok(value) => return Some(Ok(SourceTokenPropertyValue::Float(value))),
            Err(_) => return Some(Err(ArrayTokenError::ParseNumberError(index, raw_value.to_string())))
        }
    }

    fn produce_string_value_result(&mut self, start: usize, index: usize) -> ArrayTokenOption {
        let value = self.splice_input(start, index);
        return Some(Ok(SourceTokenPropertyValue::String(value.to_string())));
    }

    fn handle_inside_usize_value(&mut self, start: usize, index: usize, character: char) -> ArrayTokenOption {
        if character == ARRAY_CLOSING_CHAR {
            self.state = ArrayState::EndArray;
            return self.produce_usize_value_result(start, index);
        }
        if character == ',' {
            self.state = ArrayState::EndValue;
            return self.produce_usize_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_value(&mut self, start: usize, index: usize, character: char) -> ArrayTokenOption {
        if character == ARRAY_CLOSING_CHAR {
            self.state = ArrayState::EndArray;
            return self.produce_signed_number_value_result(start, index);
        }
        if character == ',' {
            self.state = ArrayState::EndValue;
            return self.produce_signed_number_value_result(start, index);
        }
        None
    }

    fn handle_inside_string_value(&mut self, start: usize, index: usize, character: char) -> ArrayTokenOption {
        if character == '"' {
            self.state = ArrayState::EndValue;
            return self.produce_string_value_result(start, index);
        }
        None
    }

    fn transition(&mut self, index: usize, character: char) -> ArrayTokenOption {
        match self.state {
            ArrayState::Start => {
                self.start_if_possible(index, character)
            },
            ArrayState::StartValue => {
                self.start_value_if_possible(index, character)
            },
            ArrayState::InSignedNumberValue(start) => {
                self.handle_inside_signed_number_value(start, index, character)
            },
            ArrayState::InUSizeNumberValue(start) => {
                self.handle_inside_usize_value(start, index, character)
            },
            ArrayState::InStringValue(start) => {
                self.handle_inside_string_value(start, index, character)
            },
            ArrayState::EndValue => {
                self.start_value_if_possible(index, character)
            },
            ArrayState::InWhitespace => {
                self.start_value_if_possible(index, character)
            },
            ArrayState::EndArray => {
                None
            }
        }
    }
}

impl <'a> Iterator for ArrayTokenizer<'a> {
    type Item = ArrayTokenResult;
    fn next(&mut self) -> ArrayTokenOption {
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