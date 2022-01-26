use crate::prelude::*;

impl AbstractSyntaxTreeRenderer {
    pub fn render_separator(&self, ui: &mut egui::Ui) {
        ui.separator();
    }

    pub fn render_scroll_area(&self, ui: &mut egui::Ui, props: ScrollAreaProperties, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        let scroll_area = match props.size {
            VerticalSize::Auto => self.render_auto_sized_scroll_area(ui),
            VerticalSize::MaxHeight(height) => self.render_max_height_scroll_area(ui, height),
        };

        scroll_area
            .id_source(props.id)
            .always_show_scroll(props.always_show_scroll)
            .enable_scrolling(props.enable_scrolling)
            .scroll_offset(props.scroll_offset)
            .show(ui, add_contents);
    }

    pub fn render_auto_sized_scroll_area(&self, ui: &mut egui::Ui) -> egui::ScrollArea {
        egui::ScrollArea::auto_sized()
    }

    pub fn render_max_height_scroll_area(&self, ui: &mut egui::Ui, height: f32) -> egui::ScrollArea {
        egui::ScrollArea::from_max_height(height)   
    }

    pub fn render_horizontal(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.horizontal(add_contents);
    }

    pub fn render_vertical(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.vertical(add_contents);
    }
} 

pub struct ScrollAreaProperties {
    pub id: String,
    pub size: VerticalSize,
    pub scroll_offset: f32,
    pub always_show_scroll: bool,
    pub enable_scrolling: bool,
}

impl Default for ScrollAreaProperties {
    fn default() -> Self {
        Self { 
            id: "".to_string(),
            size: VerticalSize::Auto,
            scroll_offset: 0.0,
            always_show_scroll: false,
            enable_scrolling: true,
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for ScrollAreaProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Id(value) => to.id = value.clone(),
                AbstractSyntaxTokenProperty::VerticallySized(value) => to.size = *value,
                AbstractSyntaxTokenProperty::AlwaysShowScroll(value) => to.always_show_scroll = *value,
                AbstractSyntaxTokenProperty::ScrollOffset(value) => to.scroll_offset = *value,
                AbstractSyntaxTokenProperty::EnableScrolling(value) => to.enable_scrolling = *value,
                _ => {}
            }
        }
        to
    }
}