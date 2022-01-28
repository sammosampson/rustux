use crate::prelude::*;

#[test]
fn property_with_string_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("large"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}