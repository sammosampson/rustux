
use crate::prelude::*;


#[test]
fn property_with_unsigned_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_unsigned_int_at_end_of_control_openening_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=10></rect>");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_unsigned_int_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(1)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_at_end_of_control_openening_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10></rect>");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_incorrect_unsigned_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(13)), tokenizer.next().unwrap());
}

#[test]
fn property_with_incorrect_signed_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(14)), tokenizer.next().unwrap());
}