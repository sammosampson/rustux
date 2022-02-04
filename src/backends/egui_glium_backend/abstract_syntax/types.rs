
use egui::Color32;
use crate::prelude::*;

impl From<Colour> for Color32 {
    fn from(from: Colour) -> Self {
        Self::from_rgba_unmultiplied(from.r, from.g, from.b, from.a)
    }
}

impl From<TextStyle> for egui::TextStyle {
    fn from(from: TextStyle) -> Self {
        match from {
            TextStyle::Small => egui::TextStyle::Small,
            TextStyle::Body => egui::TextStyle::Body,
            TextStyle::Button => egui::TextStyle::Button,
            TextStyle::Heading => egui::TextStyle::Heading,
            TextStyle::Monospace => egui::TextStyle::Monospace,
        }
    }
}
