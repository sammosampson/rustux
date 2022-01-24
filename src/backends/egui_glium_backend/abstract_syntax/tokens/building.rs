use crate::prelude::*;

#[derive(Debug, Default)]
pub struct BuildAbstractSyntaxSourceTokenVisitor {
    ast: AbstractSyntaxTokenStream,
    current_property_name: String
}

impl BuildAbstractSyntaxSourceTokenVisitor {
    pub fn ast(self) -> AbstractSyntaxTokenStream {
        self.ast
    }
}

impl SourceTokenVisitor for BuildAbstractSyntaxSourceTokenVisitor {
    fn token_error(&mut self, error: SourceTokenError) {
        self.ast.add_error(AbstractSyntaxTokenError::SourceTokenError(error))
    }

    fn control(&mut self, control_name: &str) {
        self.ast.start_node(match_control_name(control_name));
    }

    fn property(&mut self, property_name: &str) {
        match match_property_only(property_name) {
            Some(property) => self.ast.property(property),
            None => self.current_property_name = property_name.to_string()
        }
    }

    fn property_value(&mut self, property_value: &SourceTokenPropertyValue) {
        match match_property_value(&self.current_property_name, property_value) {
            Ok(property) => self.ast.property(property),
            Err(error) => self.ast.property_error(error),
        }
    }

    fn end_control(&mut self, control_name: &str) {
        self.ast.end_node(match_control_name(control_name));
    }
}

fn match_control_name(control_name: &str) -> AbstractSyntaxTokenType {
    match control_name {
        "root" => AbstractSyntaxTokenType::Root,
        "central-panel" => AbstractSyntaxTokenType::CentralPanel,
        "top-panel" => AbstractSyntaxTokenType::TopPanel,
        "bottom-panel" => AbstractSyntaxTokenType::BottomPanel,
        "left-side-bar" => AbstractSyntaxTokenType::LeftSidebar,
        "right-side-bar" => AbstractSyntaxTokenType::RightSidebar,
        "scroll-area" => AbstractSyntaxTokenType::ScrollArea,
        "separator" => AbstractSyntaxTokenType::Separator,
        "horizontal" => AbstractSyntaxTokenType::Horizontal,
        "vertical" => AbstractSyntaxTokenType::Vertical,
        "label" => AbstractSyntaxTokenType::Label,
        "coloured-label" => AbstractSyntaxTokenType::ColouredLabel,
        "selectable-label" => AbstractSyntaxTokenType::SelectableLabel,
        "heading" => AbstractSyntaxTokenType::Heading,
        "monospace" => AbstractSyntaxTokenType::Monospace,
        "code" => AbstractSyntaxTokenType::Code,
        _ => AbstractSyntaxTokenType::Unknown
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
                SourceTokenPropertyValue::Tuple(value) => 
                    Ok(AbstractSyntaxTokenProperty::HeightRange(FloatRange::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "width-range" => {
            match property_value {
                SourceTokenPropertyValue::Tuple(value) => 
                    Ok(AbstractSyntaxTokenProperty::WidthRange(FloatRange::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "colour" => {
            match property_value {
                SourceTokenPropertyValue::Tuple(value) => 
                    Ok(AbstractSyntaxTokenProperty::Colour(Colour::parse(value)?)),
                _ => Err(AbstractSyntaxTokenError::UnknownPropertyValue(property_name.to_string())) 
            }
        },
        "background-colour" => {
            match property_value {
                SourceTokenPropertyValue::Tuple(value) => 
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
        _ => None 
    }
}
