use crate::prelude::*;

impl AbstractSyntaxTreeRenderer {
    pub fn render_label(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(text);
    }

    pub fn render_selectable_label(&self, ui: &mut egui::Ui, text: &str, selected: bool) {
        ui.selectable_label(selected, text);
    }

    pub fn render_monospace(&self, ui: &mut egui::Ui, text: &str) {
        ui.monospace(text);
    }

    pub fn render_code(&self, ui: &mut egui::Ui, text: &str) {
        ui.code(text);
    }

    pub fn render_heading(&self, ui: &mut egui::Ui, text: &str) {
        ui.heading(text);
    }
} 