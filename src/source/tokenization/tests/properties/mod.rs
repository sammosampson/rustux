mod strings;
mod floats;
mod ints;
mod usizes;
mod booleans;
mod collections;
mod code;


use crate::prelude::*;

#[test]
fn multiple_nested_controls_with_properties_with_values_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas offset=[200, 100]><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("offset")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Array(
                vec!(
                    Ok(SourceTokenPropertyValue::USize(200)),
                    Ok(SourceTokenPropertyValue::USize(100)),
                )
            )
        ),
        tokenizer.next().unwrap().unwrap()
    );
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_properties_with_value_produces_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" edges=\"round\" other />");
    assert_eq!(SourceToken::Control(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard,  String::from(String::from("size"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from(String::from("large")))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("round"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard,  String::from(String::from("other"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}