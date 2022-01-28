use crate::prelude::*;

#[test]
fn property_without_value_produces_boolean_property_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_properties_without_value_produces_boolean_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size rounded-edges other />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("rounded-edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}


#[test]
fn multiple_nested_controls_with_valueless_properties_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas other><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
