use crate::prelude::*;

impl AbstractSyntaxTreeRenderer {
    pub fn render_separator(&self, ui: &mut egui::Ui) {
        ui.separator();
    }

    pub fn render_scroll_area(&self, ui: &mut egui::Ui, props: ScrollAreaProperties, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        let mut scroll_area = match props.size {
            None => render_auto_sized_scroll_area(),
            Some(height) => render_max_height_scroll_area(height),
        };

        scroll_area = scroll_area
            .id_source(props.id)
            .always_show_scroll(props.always_show_scroll)
            .enable_scrolling(props.enable_scrolling);
        
        if let Some(scroll_offset) = props.scroll_offset {
            scroll_area = scroll_area.scroll_offset(scroll_offset);
        }

        scroll_area.show(ui, add_contents);
    }

    pub fn render_horizontal(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.horizontal(add_contents);
    }

    pub fn render_vertical(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> ()) {
        ui.vertical(add_contents);
    }
}

fn render_auto_sized_scroll_area() -> egui::ScrollArea {
    egui::ScrollArea::auto_sized()
}

fn render_max_height_scroll_area(height: f32) -> egui::ScrollArea {
    egui::ScrollArea::from_max_height(height)   
}


pub struct ScrollAreaProperties {
    pub id: String,
    pub size: Option<f32>,
    pub scroll_offset: Option<f32>,
    pub always_show_scroll: bool,
    pub enable_scrolling: bool,
}

impl Default for ScrollAreaProperties {
    fn default() -> Self {
        Self { 
            id: "".to_string(),
            size: None,
            scroll_offset: None,
            always_show_scroll: false,
            enable_scrolling: true,
        }
    }
}

impl From<&Vec<AbstractSyntaxProperty>> for ScrollAreaProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Id => to.id = property.value().get_string_value().unwrap(),
                AbstractSyntaxPropertyType::VerticallySized => to.size = Some(property.value().get_float_value().unwrap()),
                AbstractSyntaxPropertyType::AutoSized => to.size = None,
                AbstractSyntaxPropertyType::AlwaysShowScroll => to.always_show_scroll = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::ScrollOffset => to.scroll_offset = Some(property.value().get_float_value().unwrap()),
                AbstractSyntaxPropertyType::EnableScrolling => to.enable_scrolling = property.value().get_bool_value().unwrap(),
                _ => {}
            }
        }
        to
    }
}