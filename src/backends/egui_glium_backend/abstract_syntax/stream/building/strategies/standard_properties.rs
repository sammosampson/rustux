use crate::prelude::*;

pub struct StandardBuildAbstractSyntaxTokenStreamStrategy(pub AbstractSyntaxTokenType);

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

fn match_property_value(property_name: &str, property_value: &SourceTokenPropertyValue) -> Result<AbstractSyntaxTokenProperty, AbstractSyntaxTokenError> {
    match property_name {
        "id" => {
            match property_value {
                SourceTokenPropertyValue::String(value) => Ok(AbstractSyntaxTokenProperty::Id(value.clone())),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "text" => {
            match property_value {
                SourceTokenPropertyValue::String(value) => Ok(AbstractSyntaxTokenProperty::Text(value.clone())),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "default-width" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(AbstractSyntaxTokenProperty::DefaultWidth(*value as f32)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "default-height" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(AbstractSyntaxTokenProperty::DefaultHeight(*value as f32)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "height-range" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                    Ok(AbstractSyntaxTokenProperty::HeightRange(FloatRange::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "width-range" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                    Ok(AbstractSyntaxTokenProperty::WidthRange(FloatRange::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "colour" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                    Ok(AbstractSyntaxTokenProperty::Colour(Colour::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "background-colour" => {
            match property_value {
                SourceTokenPropertyValue::Array(value) => 
                    Ok(AbstractSyntaxTokenProperty::BackgroundColour(Colour::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "text-style" => {
            match property_value {
                SourceTokenPropertyValue::String(value)
                    => Ok(AbstractSyntaxTokenProperty::TextStyle(TextStyle::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string()))
            }
        },
        "max-height" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(AbstractSyntaxTokenProperty::VerticallySized(VerticalSize::MaxHeight(*value as f32))),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "scroll_offset" => {
            match property_value {
                SourceTokenPropertyValue::Float(value) => Ok(AbstractSyntaxTokenProperty::ScrollOffset(*value as f32)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "on-select" => {
            match property_value {
                SourceTokenPropertyValue::Code(value) => Ok(AbstractSyntaxTokenProperty::OnSelect(Function::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        _ => Err(AbstractSyntaxTokenError::UnknownProperty(property_name.to_string())) 
    }
}

fn match_property_only(property_name: &str) -> Option<AbstractSyntaxTokenProperty> {
    match property_name {
        "selected" => Some(AbstractSyntaxTokenProperty::Selected(true)),
        "resizable" => Some(AbstractSyntaxTokenProperty::Resizable(true)),
        "wrap" => Some(AbstractSyntaxTokenProperty::Wrap(true)),
        "code" => Some(AbstractSyntaxTokenProperty::Code(true)),
        "strong" => Some(AbstractSyntaxTokenProperty::Strong(true)),
        "weak" => Some(AbstractSyntaxTokenProperty::Weak(true)),
        "strike-through" => Some(AbstractSyntaxTokenProperty::Strikethrough(true)),
        "underline" => Some(AbstractSyntaxTokenProperty::Underline(true)),
        "italics" => Some(AbstractSyntaxTokenProperty::Italics(true)),
        "raised" => Some(AbstractSyntaxTokenProperty::Raised(true)),
        "auto-sized" => Some(AbstractSyntaxTokenProperty::VerticallySized(VerticalSize::Auto)),
        "always_show_scroll" => Some(AbstractSyntaxTokenProperty::AlwaysShowScroll(true)),
        "enable_scrolling" => Some(AbstractSyntaxTokenProperty::EnableScrolling(true)),
        _ => None 
    }
}