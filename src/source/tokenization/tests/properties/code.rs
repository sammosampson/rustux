use crate::prelude::*;


#[test]
fn property_with_variable_property_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect position=$item_x />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("position")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Variable(String::from("item_x"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_variable_property_value_in_opening_control_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect position=$item_x></rect>");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("position")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Variable(String::from("item_x"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

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
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::USize(1))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::USize(2))),
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
fn property_with_function_value_containing_variable_argument_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect on-click={click_it($item)} />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("on-click")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Code(
                vec!(
                    Ok(CodeTokenPropertyValue::StartFunction(String::from("click_it"))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::Variable(String::from("item")))),
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
fn property_with_multi_argument_function_value_containing_variable_argument_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect on-click={click_it($item, 2)} />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(SourceTokenPropertyType::Standard, String::from("on-click")), tokenizer.next().unwrap().unwrap());
    
    assert_eq!(
        SourceToken::PropertyValue(
            SourceTokenPropertyValue::Code(
                vec!(
                    Ok(CodeTokenPropertyValue::StartFunction(String::from("click_it"))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::Variable(String::from("item")))),
                    Ok(CodeTokenPropertyValue::PropertyValue(SourceTokenPropertyValue::USize(2))),
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
                    Ok(SourceTokenPropertyValue::USize(1)),
                    Ok(SourceTokenPropertyValue::USize(2))
                )
            )
        ),
        tokenizer.next().unwrap().unwrap()
    );

    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}