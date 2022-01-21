use egui_glium::*;

use crate::prelude::*;

pub struct EguiRenderer {
    egui: EguiGlium
}

impl EguiRenderer {
    pub fn new(display: &Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }

    pub fn set_visuals(&mut self) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
    }

    pub fn begin_frame(&mut self, display: &Display) {
        self.egui.begin_frame(display);
    }

    pub fn end_frame_and_paint(&mut self, display: &Display, target: &mut Frame) -> bool {
        let (needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);
        needs_repaint
    }

    pub fn render_side_panel(&self, name: &str, orientation: HorizontalOrientation, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        match orientation {
            HorizontalOrientation::Left => self.render_left_side_panel(name, add_contents),
            HorizontalOrientation::Right => self.render_right_side_panel(name, add_contents),
        }
    }
    
    fn render_left_side_panel(&self, name: &str, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::SidePanel::left(name)
            .resizable(false)
            .show(self.egui.ctx(), add_contents);
    }

    fn render_right_side_panel(&self, name: &str, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::SidePanel::right(name)
            .resizable(false)
            .show(self.egui.ctx(), add_contents);
    }

    pub fn render_scroll_area(&self, size: VerticalSize, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        match size {
            VerticalSize::Auto => self.render_auto_sized_scroll_area(ui, add_contents),
            VerticalSize::MaxHeight(height) => self.render_max_height_scroll_area(height, ui, add_contents),
        }
    }

    pub fn render_auto_sized_scroll_area(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::ScrollArea::auto_sized().show(ui, add_contents);
    }

    pub fn render_max_height_scroll_area(&self, height: f32, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::ScrollArea::from_max_height(height).show(ui, add_contents);        
    }

    pub fn render_horizontal(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.horizontal(add_contents);
    }

    pub fn render_vertical(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.vertical(add_contents);
    }

    pub fn render_label(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(text);
    }
}