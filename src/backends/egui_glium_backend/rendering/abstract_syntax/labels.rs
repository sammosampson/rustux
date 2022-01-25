use crate::prelude::*;
use egui::{Color32, TextStyle};

impl AbstractSyntaxTreeRenderer {
    pub fn render_label(&self, ui: &mut egui::Ui, props: LabelProperties) {
        let mut label = egui::Label::new(props.text);

        if let Some(wrap) = props.wrap {
            label = label.wrap(wrap)
        }

        if let Some(text_style) = props.text_style {
            label = label.text_style(text_style)
        }

        if let Some(text_color) = props.text_color {
            label = label.text_color(text_color)
        }

        label = label.background_color(props.background_color);

        if props.code {
            label = label.code();
        }
        if props.strong {
            label = label.strong();
        }
        if props.weak {
            label = label.weak();
        }
        if props.strikethrough {
            label = label.strikethrough();
        }
        if props.underline {
            label = label.underline();
        }
        if props.italics {
            label = label.italics();
        }
        if props.raised {
            label = label.raised();
        }

        ui.add(label);
    }

    pub fn render_coloured_label(&self, ui: &mut egui::Ui, props: ColouredLabelProperties) {
        ui.colored_label(props.colour, props.text);
    }

    pub fn render_selectable_label(&self, ui: &mut egui::Ui, props: SelectableLabelProperties) {
        let selectable_label = ui.selectable_label(props.selected, props.text);
    }

    pub fn render_monospace(&self, ui: &mut egui::Ui, props: MonospaceProperties) {
        ui.monospace(props.text);
    }

    pub fn render_code(&self, ui: &mut egui::Ui, props: CodeProperties) {
        ui.code(props.text);
    }

    pub fn render_heading(&self, ui: &mut egui::Ui, props: HeadingProperties) {
        ui.heading(props.text);
    }
}

pub struct LabelProperties {
    pub text: String,
    pub wrap: Option<bool>,
    pub text_style: Option<TextStyle>,
    pub background_color: Color32,
    pub text_color: Option<Color32>,
    pub code: bool,
    pub strong: bool,
    pub weak: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub italics: bool,
    pub raised: bool,
}

impl Default for LabelProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string(),
            wrap: None,
            text_style: None,
            background_color: Color32::TRANSPARENT,
            text_color: None,
            code: false,
            strong: false,
            weak: false,
            strikethrough: false,
            underline: false,
            italics: false,
            raised: false,
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for LabelProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.into(),
                AbstractSyntaxTokenProperty::Wrap(value) => to.wrap = Some(*value),
                AbstractSyntaxTokenProperty::TextStyle(value) => to.text_style = Some(value.into()),
                AbstractSyntaxTokenProperty::BackgroundColour(value) => to.background_color = value.into(),
                AbstractSyntaxTokenProperty::Colour(value) => to.text_color = Some(value.into()),
                AbstractSyntaxTokenProperty::Code(value) => to.code = *value,
                AbstractSyntaxTokenProperty::Strong(value) => to.strong = *value,
                AbstractSyntaxTokenProperty::Weak(value) => to.weak = *value,
                AbstractSyntaxTokenProperty::Strikethrough(value) => to.strikethrough = *value,
                AbstractSyntaxTokenProperty::Underline(value) => to.underline = *value,
                AbstractSyntaxTokenProperty::Italics(value) => to.italics = *value,
                AbstractSyntaxTokenProperty::Raised(value) => to.raised = *value,
                _ => {}
            }
        }
        to
    }
}
pub struct ColouredLabelProperties {
    pub colour: Color32,
    pub text: String
}

impl Default for ColouredLabelProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string(),
            colour: Color32::TRANSPARENT,
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for ColouredLabelProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.into(),
                AbstractSyntaxTokenProperty::Colour(value) => to.colour = value.into(),
                _ => {}
            }
        }
        to
    }
}

pub struct SelectableLabelProperties {
    pub text: String,
    pub selected: bool
}

impl Default for SelectableLabelProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string(),
            selected: false,
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for SelectableLabelProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.clone(),
                AbstractSyntaxTokenProperty::Selected(value) => to.selected = *value,
                _ => {}
            }
        }
        to
    }
}

pub struct MonospaceProperties {
    pub text: String
}

impl Default for MonospaceProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string()
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for MonospaceProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.clone(),
                _ => {}
            }
        }
        to
    }
}

pub struct CodeProperties {
    pub text: String
}

impl Default for CodeProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string()
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for CodeProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.clone(),
                _ => {}
            }
        }
        to
    }
}

pub struct HeadingProperties {
    pub text: String
}

impl Default for HeadingProperties {
    fn default() -> Self {
        Self { 
            text: "".to_string()
        }
    }
}

impl From<&Vec<AbstractSyntaxTokenProperty>> for HeadingProperties {
    fn from(from: &Vec<AbstractSyntaxTokenProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property {
                AbstractSyntaxTokenProperty::Text(value) => to.text = value.clone(),
                _ => {}
            }
        }
        to
    }
}
