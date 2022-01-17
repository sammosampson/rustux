use std::iter::Enumerate;
use std::str::Chars;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TupleValue<'a> {
    String(&'a str),
    Float(f64),
    Int(i128),
    UnsignedInt(u128)
}

#[derive(PartialEq, Eq, Debug)]
enum TupleState {
    Start,
    StartValue,
    EndTuple,
    InSignedNumberValue(usize),
    InUnsignedNumberValue(usize),
    EndValue,
    InWhitespace
}

#[derive(PartialEq, Eq, Debug)]
pub enum TupleTokenError<'a> {
    TupleError(&'a str, usize, char),
    ValueError(&'a str, usize, &'a str),
}

impl<'a> TupleTokenError<'a> {
    pub fn could_not_find_opening_parentheses(index: usize, character: char) -> Self {
        TupleTokenError::TupleError("could not find opening parentheses '('", index, character)
    }

    pub fn could_not_find_closing_parentheses(index: usize, character: char) -> Self {
        TupleTokenError::TupleError("could not find closing parentheses ')'", index, character)
    }

    pub fn could_not_parse_number_value(index: usize, value: &'a str) -> Self {
        TupleTokenError::ValueError("could not parse number value", index, value)
    }
}

pub struct TupleTokenFloatIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>> {
    token_iterator: I
}

impl<'a, I> TupleTokenFloatIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>> {
    pub fn from_iterator(token_iterator: I) -> Self {
        Self {
            token_iterator
        }
    }
}

impl <'a, I> Iterator for TupleTokenFloatIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>>{
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        loop {
            return match self.token_iterator.next() {
                Some(result) => if let Ok(TupleValue::Float(value)) = result {
                    Some(value as f32)
                } else {
                    None
                },
                None => None
            }
        }
    }
}

pub struct TupleTokenUnsignedShortIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>> {
    token_iterator: I
}

impl<'a, I> TupleTokenUnsignedShortIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>> {
    pub fn from_iterator(token_iterator: I) -> Self {
        Self {
            token_iterator
        }
    }
}

impl <'a, I> Iterator for TupleTokenUnsignedShortIterator<'a, I> where I : Iterator<Item=TupleTokenResult<'a>>{
    type Item = u16;
    fn next(&mut self) -> Option<u16> {
        loop {
            return match self.token_iterator.next() {
                Some(result) => if let Ok(TupleValue::UnsignedInt(value)) = result {
                    Some(value as u16)
                } else {
                    None
                },
                None => None
            }
        }
    }
}

pub type TupleTokenResult<'a> = Result<TupleValue<'a>, TupleTokenError<'a>>;
pub type TupleTokenOption<'a> = Option<TupleTokenResult<'a>>;

pub struct TupleTokenizer<'a>{
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    state: TupleState
}

impl<'a> TupleTokenizer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: TupleState::Start
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char) -> TupleTokenOption<'a> {
        if character == '(' {
            self.state = TupleState::StartValue;
            return None;
        }
        Some(Err(TupleTokenError::could_not_find_opening_parentheses(index, character)))
    }

    fn start_value_if_possible(&mut self, index: usize, character: char) -> TupleTokenOption<'a> {
        if character == ')' {
            self.state = TupleState::EndTuple;
            return None;
        }
        if character.is_numeric() {
            self.state = TupleState::InUnsignedNumberValue(index);
            return None;
        }
        if character == '-' {
            self.state = TupleState::InSignedNumberValue(index);
            return None;
        }
        if character == ' ' {
            self.state = TupleState::InWhitespace;
            return None;
        }
        Some(Err(TupleTokenError::could_not_find_closing_parentheses(index, character)))
    }
    
    fn produce_signed_number_value_result(&mut self, start: usize, index: usize) -> TupleTokenOption<'a> {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<i128>() {
            Ok(value) => return Some(Ok(TupleValue::Int(value))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_unsigned_number_value_result(&mut self, start: usize, index: usize) -> TupleTokenOption<'a> {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<u128>() {
            Ok(value) => return Some(Ok(TupleValue::UnsignedInt(value))),
            Err(_) => return self.produce_float_value_result(raw_value, index)
        }
    }

    fn produce_float_value_result(&mut self, raw_value: &'a str, index: usize) -> TupleTokenOption<'a> {
        match raw_value.parse::<f64>() {
            Ok(value) => return Some(Ok(TupleValue::Float(value))),
            Err(_) => return Some(Err(TupleTokenError::could_not_parse_number_value(index, raw_value)))
        }
    }

    fn handle_inside_unsigned_number_value(&mut self, start: usize, index: usize, character: char) -> TupleTokenOption<'a> {
        if character == ')' {
            self.state = TupleState::EndTuple;
            return self.produce_unsigned_number_value_result(start, index);
        }
        if character == ',' {
            self.state = TupleState::EndValue;
            return self.produce_unsigned_number_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_value(&mut self, start: usize, index: usize, character: char) -> TupleTokenOption<'a> {
        if character == ')' {
            self.state = TupleState::EndTuple;
            return self.produce_signed_number_value_result(start, index);
        }
        if character == ',' {
            self.state = TupleState::EndValue;
            return self.produce_signed_number_value_result(start, index);
        }
        None
    }

    fn transition(&mut self, index: usize, character: char) -> TupleTokenOption<'a> {
        match self.state {
            TupleState::Start => {
                self.start_if_possible(index, character)
            },
            TupleState::StartValue => {
                self.start_value_if_possible(index, character)
            },
            TupleState::InSignedNumberValue(start) => {
                self.handle_inside_signed_number_value(start, index, character)
            },
            TupleState::InUnsignedNumberValue(start) => {
                self.handle_inside_unsigned_number_value(start, index, character)
            },
            TupleState::EndValue => {
                self.start_value_if_possible(index, character)
            },
            TupleState::InWhitespace => {
                self.start_value_if_possible(index, character)
            },
            TupleState::EndTuple => {
                None
            }
        }
    }
}

impl <'a> Iterator for TupleTokenizer<'a> {
    type Item = TupleTokenResult<'a>;
    fn next(&mut self) -> TupleTokenOption<'a> {
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