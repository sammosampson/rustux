use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SourceTokenError {
    CouldNotFindStartTag(usize),
    CouldNotParseNumberValue(usize),
    CouldNotFindControlName(usize),
    CouldNotFindPropertyStartSymbol(usize),
    CouldNotFindControlToClose(usize),
    CouldNotFindControlCloseSymbol(usize),
    ClosingWrongTag(usize)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceTokenPropertyValue {
    String(String),
    Int(i128),
    UnsignedInt(u128),
    Float(f64),
    Tuple(String)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceToken {
    Control(String),
    EndControl(String),
    Property(String),
    PropertyValue(SourceTokenPropertyValue),
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Start,
    StartControl,
    InControl(usize),
    EndControl,
    EndNestedControl(usize),
    InProperty(usize),
    InStringPropertyValue(usize),
    InUnsignedNumberPropertyValue(usize),
    InSignedNumberPropertyValue(usize),
    InTuplePropertyValue(usize),
    StartPropertyValue,
    InWhitespace
} 

pub type SourceTokenResult = Result<SourceToken, SourceTokenError>;
pub type SourceTokenOption = Option<SourceTokenResult>;

pub struct SourceTokenizer<'a> {
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    current_parent: Vec<&'a str>,
    state: State
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
            state: State::Start,
            current_parent: vec![]
        }
    }

    fn transition(&mut self, index: usize, character: char) -> SourceTokenOption {
        match self.state {
            State::Start => {
                self.start_if_possible(index, character)
            },
            State::StartControl => {
                self.start_control_if_possible(index, character)
            },
            State::InControl(start) => {
                self.handle_inside_control(start, index, character)
            },
            State::EndControl => {
                self.end_control_if_possible(index, character)
            },
            State::EndNestedControl(start) => {
                self.end_nested_control_if_possible(start, index, character)
            },
            State::InProperty(start) => {
                self.handle_inside_property(start, index, character)
            },
            State::StartPropertyValue => {
                self.start_property_value_if_possible(index, character)
            },
            State::InStringPropertyValue(start) => {
                self.handle_inside_string_property_value(start, index, character)
            },
            State::InUnsignedNumberPropertyValue(start) => {
                self.handle_inside_unsigned_number_property_value(start, index, character)
            },
            State::InSignedNumberPropertyValue(start) => {
                self.handle_inside_signed_number_property_value(start, index, character)
            },
            State::InTuplePropertyValue(start) => {
                self.handle_inside_tuple_property_value(start, index, character)
            },
            State::InWhitespace => {
                self.handle_inside_whitespace(index, character)
            }
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '<' {
            self.state = State::StartControl;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
        Some(Err(SourceTokenError::CouldNotFindStartTag(index)))
    }
    
    fn start_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '/' {
            self.state = State::EndNestedControl(index + 1);
            return None;
        }
        if !character.is_whitespace() {
            self.state = State::InControl(index);
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
            self.state = State::InWhitespace;
            return self.produce_control_result(start,index);
        }
        if character == '>' {
            self.state = State::Start;
            return self.produce_control_result(start,index);
        }
        if character == '/' {
            self.state = State::EndControl;
            return self.produce_control_result(start,index);
        }
        None
    }

    fn produce_property_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        Some(Ok(SourceToken::Property(String::from(self.splice_input(start, index)))))
    }

    fn handle_inside_property(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character.is_whitespace() {
            self.state = State::InWhitespace;
            return self.produce_property_result(start,index);
        }
        if character == '/' {
            self.state = State::EndControl;
            return self.produce_property_result(start,index);
        }
        if character == '=' {
            self.state = State::StartPropertyValue;
            return self.produce_property_result(start,index);
        }
        if character == '>' {
            self.state = State::Start;
            return self.produce_property_result(start,index);
        }
        None
    }
    
    fn start_property_value_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '"' {
            self.state = State::InStringPropertyValue(index + 1);
            return None;
        }
        if character.is_numeric() {
            self.state = State::InUnsignedNumberPropertyValue(index);
            return None;
        }
        if character == '-' {
            self.state = State::InSignedNumberPropertyValue(index);
            return None;
        }
        if character == '(' {
            self.state = State::InTuplePropertyValue(index);
            return None;
        }
        Some(Err(SourceTokenError::CouldNotFindPropertyStartSymbol(index)))
    }  

    fn produce_string_property_value_result(&mut self, start: usize, index: usize)  -> SourceTokenOption {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from(self.splice_input(start, index))))))
    }

    fn produce_unsigned_number_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<u128>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(value)))),
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

    fn produce_tuple_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(String::from(self.splice_input(start, index))))))
    }
    
    fn handle_inside_string_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == '"' {
            self.state = State::InWhitespace;
            return self.produce_string_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_unsigned_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ' ' || character.is_whitespace() {
            self.state = State::InWhitespace;
            return self.produce_unsigned_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ' ' || character.is_whitespace()  {
            self.state = State::InWhitespace;
            return self.produce_signed_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_tuple_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == ')' {
            self.state = State::InWhitespace;
            return self.produce_tuple_property_value_result(start, index + 1);
        }
        None
    }

    fn end_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption {
        if character == '>' {
            self.state = State::Start;
            match self.current_parent.pop() {
                Some(control_name) => return Some(Ok(SourceToken::EndControl(String::from(control_name)))),
                None => return Some(Err(SourceTokenError::CouldNotFindControlToClose(index)))
            };
        }
        Some(Err(SourceTokenError::CouldNotFindControlCloseSymbol(index)))
    }

    fn end_nested_control_if_possible(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption {
        if character == '>' {
            self.state = State::Start;
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
            self.state = State::EndControl;
            return None;
        }
        if character == '>' {
            self.state = State::Start;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
                  
        self.state = State::InProperty(index);
        None
    }    
}

#[test]
fn single_control_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_whitespace_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_carriage_returns_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect
    />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn incorrect_opening_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("X");
    assert_eq!(Err(SourceTokenError::CouldNotFindStartTag(0)), tokenizer.next().unwrap());
}

#[test]
fn whitespace_after_token_opening_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("< rect/>");
    assert_eq!(Err(SourceTokenError::CouldNotFindControlName(1)), tokenizer.next().unwrap());
}

#[test]
fn incorrect_closing_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/X");
    tokenizer.next();
    assert_eq!(Err(SourceTokenError::CouldNotFindControlCloseSymbol(6)), tokenizer.next().unwrap());
}

#[test]
fn multiple_consecutive_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/><circle/><line/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas><circle><line/></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_with_valueless_properties_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas other><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_with_properties_with_values_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas offset=(200, 100)><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("offset")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(String::from("(200, 100)"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn control_with_incorrect_closing_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect><line></line></circle>");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::ClosingWrongTag(27)), tokenizer.next().unwrap());
}

#[test]
fn control_with_incorrect_closing_final_bracket_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect></rect/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::ClosingWrongTag(13)), tokenizer.next().unwrap());
}

#[test]
fn property_without_value_produces_boolean_property_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_string_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("large"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_unsigned_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_number_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(1)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
#[test]
fn property_with_int_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_negative_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1.0 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(-1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_positive_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1.0 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_tuple_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=(1.0, 1.0) />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(String::from("(1.0, 1.0)"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_incorrect_unsigned_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(13)), tokenizer.next().unwrap());
}

#[test]
fn property_with_incorrect_signed_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(14)), tokenizer.next().unwrap());
}

#[test]
fn multiple_properties_without_value_produces_boolean_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size rounded-edges other />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("rounded-edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_properties_with_value_produces_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" edges=\"round\" other />");
    assert_eq!(SourceToken::Control(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from(String::from("size"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from(String::from("large")))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("round"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from(String::from("other"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

pub trait SourceTokenVisitor {
    fn token_error(&mut self, error: SourceTokenError);
    fn control(&mut self, control_name: &str);
    fn property(&mut self, property_name: &str);
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
                    SourceToken::Property(property_name) => visitor.property(&property_name),
                    SourceToken::PropertyValue(property_value) => visitor.property_value(&property_value),
                    SourceToken::EndControl(control_name) => visitor.end_control(&control_name),
                },
                Err(error) => visitor.token_error(error),
            }
        }
    }
}