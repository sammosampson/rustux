
use crate::prelude::*;

impl AbstractSyntaxTreeRenderer {
    pub fn render_central_panel(&self, contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::CentralPanel::default()
            .show(self.egui.ctx(), contents);
    }

    pub fn render_left_side_panel(&self, props: SidePanelProperties, contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::SidePanel::left(props.id)
            .resizable(props.resizable)
            .default_width(props.default_width)
            .width_range(props.width_range)
            .show(self.egui.ctx(), contents);
    }

    pub fn render_right_side_panel(&self, props: SidePanelProperties, contents: impl FnOnce(&mut egui::Ui) -> ()) {
        egui::SidePanel::right(props.id)
            .resizable(props.resizable)
            .default_width(props.default_width)
            .width_range(props.width_range)
            .show(self.egui.ctx(), contents);
    }

    pub fn render_top_panel(&self, props: TopBottomPanelProperties, contents: impl FnOnce(&mut egui::Ui) -> ()) {
        self.render_top_bottom_panel(&props, egui::TopBottomPanel::top(&props.id), contents);
    }

    pub fn render_bottom_panel(&self, props: TopBottomPanelProperties, contents: impl FnOnce(&mut egui::Ui) -> ()) {
        self.render_top_bottom_panel(&props, egui::TopBottomPanel::bottom(&props.id), contents);
    }

    fn render_top_bottom_panel(
        &self,
        props: &TopBottomPanelProperties,
        mut panel: egui::TopBottomPanel,
        contents: impl FnOnce(&mut egui::Ui)
    ) {
        if let Some(default_height) = props.default_height {
            panel = panel.default_height(default_height)
        }
        panel
            .height_range(props.height_range.clone())
            .show(self.egui.ctx(), contents);
    }
}

pub struct TopBottomPanelProperties {
    pub id: String,
    pub resizable: bool,
    pub default_height: Option<f32>,
    pub height_range: RangeInclusive<f32>,
}

impl Default for TopBottomPanelProperties {
    fn default() -> Self {
        Self { 
            id: "".to_string(),
            resizable: false,
            default_height: None,
            height_range: 96.0..=f32::INFINITY
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for TopBottomPanelProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Id(value) => to.id = value.clone(),
                AbstractSyntaxTokenProperty::Resizable(value) => to.resizable = *value,
                AbstractSyntaxTokenProperty::DefaultHeight(value) => to.default_height = Some(*value),
                AbstractSyntaxTokenProperty::HeightRange(value) => to.height_range = RangeInclusive::<f32>::from(value),
                _ => {}
            }
        }
        to
    }
}

pub struct SidePanelProperties {
    pub id: String,
    pub resizable: bool,
    pub default_width: f32,
    pub width_range: RangeInclusive<f32>,
}

impl Default for SidePanelProperties {
    fn default() -> Self {
        Self { 
            id: "".to_string(),
            resizable: false,
            default_width: 200.0,
            width_range: 96.0..=f32::INFINITY
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for SidePanelProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Id(value) => to.id = value.clone(),
                AbstractSyntaxTokenProperty::Resizable(value) => to.resizable = *value,
                AbstractSyntaxTokenProperty::DefaultWidth(value) => to.default_width = *value,
                AbstractSyntaxTokenProperty::WidthRange(value) => to.width_range = RangeInclusive::<f32>::from(value),
                _ => {}
            }
        }
        to
    }
}
