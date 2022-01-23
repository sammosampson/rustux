use crate::prelude::*;

impl AbstractSyntaxTreeRenderer {
    pub fn render_separator(&self, ui: &mut egui::Ui) {
        ui.separator();
    }

    pub fn render_scroll_area(&self, id: &str, size: VerticalSize, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        match size {
            VerticalSize::Auto => self.render_auto_sized_scroll_area(id, ui, add_contents),
            VerticalSize::MaxHeight(height) => self.render_max_height_scroll_area(id, height, ui, add_contents),
        }
    }

    pub fn render_auto_sized_scroll_area(&self, id: &str, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::ScrollArea::auto_sized().id_source(id).show(ui, add_contents);
    }

    pub fn render_max_height_scroll_area(&self, id: &str, height: f32, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::ScrollArea::from_max_height(height).id_source(id).show(ui, add_contents);        
    }

    pub fn render_horizontal(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.horizontal(add_contents);
    }

    pub fn render_vertical(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.vertical(add_contents);
    }
} 