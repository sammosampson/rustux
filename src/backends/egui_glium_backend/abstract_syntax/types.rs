
use egui::Color32;
use crate::prelude::*;

impl From<Colour> for Color32 {
    fn from(from: Colour) -> Self {
        Self::from_rgba_unmultiplied(from.r, from.g, from.b, from.a)
    }
}

impl From<&AbstractSyntaxPropertyValue> for egui::TextStyle {
    fn from(from: &AbstractSyntaxPropertyValue) -> Self {
        match from {
            AbstractSyntaxPropertyValue::String(value) => 
                match value.as_str() { 
                    "small" => egui::TextStyle::Small,
                    "body" => egui::TextStyle::Body,
                    "button" => egui::TextStyle::Button,
                    "heading" => egui::TextStyle::Heading,
                    "monospace" => egui::TextStyle::Monospace,
                    _=> panic!()
            },
            _ => panic!()
        }
    }
}
