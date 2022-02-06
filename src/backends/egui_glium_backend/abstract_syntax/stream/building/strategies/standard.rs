use crate::prelude::*;

pub struct StandardBuildAbstractSyntaxTokenStreamStrategy(pub AbstractSyntaxControlType);

impl BuildAbstractSyntaxTokenStreamStrategy for StandardBuildAbstractSyntaxTokenStreamStrategy {
    fn control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.start_node(self.0);
    }

    fn property(&self, property: &CurrentProperty, ast: &mut AbstractSyntaxTokenStream) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(property_name) => {
                if let Some(property) = match_property_only(&property_name) {
                    ast.property(property);
                }
            },
            CurrentProperty::Variable(_) => {}
        }
    }

    fn property_value(&self, property: &CurrentProperty, property_value: &SourceTokenPropertyValue, ast: &mut AbstractSyntaxTokenStream) {
        match property {
            CurrentProperty::None => {},
            CurrentProperty::Standard(current_property_name) => {
                match match_property_value(&current_property_name, property_value) {
                    Ok(property) => ast.property(property),
                    Err(error) => ast.property_error(error),
                }
            },
            CurrentProperty::Variable(variable_name) =>
                ast.property_error(AbstractSyntaxTokenError::UnknownProperty(variable_name.to_string())),
        }
    }

    fn end_control(&self, ast: &mut AbstractSyntaxTokenStream) {
        ast.end_node(self.0);
    }
}

fn match_property_value(property_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxProperty, AbstractSyntaxTokenError> {
    match property_name {
        "id" => {
            match property_value {
                SourceTokenPropertyValue::String(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::Id, 
                    AbstractSyntaxPropertyValue::String(value.clone())
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "text" => {
            match property_value {
                SourceTokenPropertyValue::String(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::Text, 
                    AbstractSyntaxPropertyValue::String(value.clone())
                )),
                SourceTokenPropertyValue::Variable(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::Text, 
                    AbstractSyntaxPropertyValue::VariablePath(VariablePath::parse(value.clone())?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "default-width" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::DefaultWidth, 
                    AbstractSyntaxPropertyValue::Float(*value as f32)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "default-height" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::DefaultHeight, 
                    AbstractSyntaxPropertyValue::Float(*value as f32)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "height-range" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                Ok(create_ast_property(
                    AbstractSyntaxPropertyType::HeightRange, 
                    AbstractSyntaxPropertyValue::FloatRange(FloatRange::parse(value)?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "width-range" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                Ok(create_ast_property(
                    AbstractSyntaxPropertyType::WidthRange, 
                    AbstractSyntaxPropertyValue::FloatRange(FloatRange::parse(value)?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "colour" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                    Ok(create_ast_property(
                        AbstractSyntaxPropertyType::Colour, 
                        AbstractSyntaxPropertyValue::Colour(Colour::parse(value)?)
                    )
                ),
                SourceTokenPropertyValue::Variable(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::Colour, 
                    AbstractSyntaxPropertyValue::VariablePath(VariablePath::parse(value.clone())?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "background-colour" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                Ok(create_ast_property(
                    AbstractSyntaxPropertyType::BackgroundColour, 
                    AbstractSyntaxPropertyValue::Colour(Colour::parse(value)?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "text-style" => {
            match property_value {
                SourceTokenPropertyValue::String(value) =>
                    Ok(create_ast_property(
                        AbstractSyntaxPropertyType::TextStyle, 
                        AbstractSyntaxPropertyValue::String(value.clone())
                    )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string()))
            }
        },
        "max-height" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => 
                Ok(create_ast_property(
                    AbstractSyntaxPropertyType::VerticallySized, 
                    AbstractSyntaxPropertyValue::Float(*value as f32)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "scroll_offset" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::ScrollOffset, 
                    AbstractSyntaxPropertyValue::Float(*value as f32)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "on-select" => {
            match property_value {
                SourceTokenPropertyValue::Code(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::OnSelect, 
                    AbstractSyntaxPropertyValue::Function(Function::parse(value)?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "selected" => {
            match property_value {
                SourceTokenPropertyValue::Variable(value) => Ok(create_ast_property(
                    AbstractSyntaxPropertyType::Selected, 
                    AbstractSyntaxPropertyValue::VariablePath(VariablePath::parse(value.clone())?)
                )),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(property_name.to_string())) 
    }
}

fn match_property_only(property_name: &str) -> Option<AbstractSyntaxProperty> {
    match property_name {
        "selected" => Some(create_ast_property(AbstractSyntaxPropertyType::Selected, AbstractSyntaxPropertyValue::Bool(true))),
        "resizable" => Some(create_ast_property(AbstractSyntaxPropertyType::Resizable, AbstractSyntaxPropertyValue::Bool(true))),
        "wrap" => Some(create_ast_property(AbstractSyntaxPropertyType::Wrap, AbstractSyntaxPropertyValue::Bool(true))),
        "code" => Some(create_ast_property(AbstractSyntaxPropertyType::Code, AbstractSyntaxPropertyValue::Bool(true))),
        "strong" => Some(create_ast_property(AbstractSyntaxPropertyType::Strong, AbstractSyntaxPropertyValue::Bool(true))),
        "weak" => Some(create_ast_property(AbstractSyntaxPropertyType::Weak, AbstractSyntaxPropertyValue::Bool(true))),
        "strike-through" => Some(create_ast_property(AbstractSyntaxPropertyType::Strikethrough, AbstractSyntaxPropertyValue::Bool(true))),
        "underline" => Some(create_ast_property(AbstractSyntaxPropertyType::Underline, AbstractSyntaxPropertyValue::Bool(true))),
        "italics" => Some(create_ast_property(AbstractSyntaxPropertyType::Italics, AbstractSyntaxPropertyValue::Bool(true))),
        "raised" => Some(create_ast_property(AbstractSyntaxPropertyType::Raised, AbstractSyntaxPropertyValue::Bool(true))),
        "auto-sized" => Some(create_ast_property(AbstractSyntaxPropertyType::AutoSized, AbstractSyntaxPropertyValue::Bool(true))),
        "always_show_scroll" => Some(create_ast_property(AbstractSyntaxPropertyType::AlwaysShowScroll, AbstractSyntaxPropertyValue::Bool(true))),
        "enable_scrolling" => Some(create_ast_property(AbstractSyntaxPropertyType::EnableScrolling, AbstractSyntaxPropertyValue::Bool(true))),
        _ => None 
    }
}