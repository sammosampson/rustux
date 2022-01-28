use crate::prelude::*;

#[test]
fn property_with_function_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect on-click={click_it(1, 2)} />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("on-click")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Code(
                vec!(
                    Ok(CodeTokenPropertyValue::StartFunction(String::from("click_it"))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::UnsignedInt(1))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::UnsignedInt(2))),
                    Ok(CodeTokenPropertyValue::EndFunction),

                )
            )
        ),
        tokenizer.next().unwrap().unwrap()
    );

    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn variable_property_with_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect $x=[1, 2] />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Variable, String::from("x")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Array(
                vec!(
                    Ok(SourceTokenPropertyValue::UnsignedInt(1)),
                    Ok(SourceTokenPropertyValue::UnsignedInt(2))
                )
            )
        ),
        tokenizer.next().unwrap().unwrap()
    );

    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}