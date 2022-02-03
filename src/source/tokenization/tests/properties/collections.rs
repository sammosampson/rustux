use crate::prelude::*;

#[test]
pub fn property_with_array_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=[1.0, 2, -5, \"xxx\"] />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("size")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Array(
                vec!(
                    Ok(SourceTokenPropertyValue::Float(1.0)),
                    Ok(SourceTokenPropertyValue::USize(2)),
                    Ok(SourceTokenPropertyValue::Int(-5)),
                    Ok(SourceTokenPropertyValue::String(String::from("xxx")))
                )
            )
        ),
        tokenizer.next().unwrap().unwrap()
    );

    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
