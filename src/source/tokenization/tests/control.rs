use crate::prelude::*;

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