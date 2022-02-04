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

    pub fn render_selectable_label(&self, ui: &mut egui::Ui, context: &mut DataContext, props: SelectableLabelProperties) {
        let response = ui.selectable_label(props.selected, props.text);
        if response.clicked() {
            context.run_action_function(&props.on_selected).unwrap();
        }
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

impl From<&Vec<AbstractSyntaxProperty>> for LabelProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
                AbstractSyntaxPropertyType::Wrap => to.wrap = Some(property.value().get_bool_value().unwrap()),
                AbstractSyntaxPropertyType::TextStyle => to.text_style = Some(property.value().get_text_style_value().unwrap().into()),
                AbstractSyntaxPropertyType::BackgroundColour => to.background_color = property.value().get_colour_value().unwrap().into(),
                AbstractSyntaxPropertyType::Colour => to.text_color = Some(property.value().get_colour_value().unwrap().into()),
                AbstractSyntaxPropertyType::Code => to.code = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Strong => to.strong = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Weak => to.weak = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Strikethrough => to.strikethrough = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Underline => to.underline = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Italics => to.italics = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::Raised => to.raised = property.value().get_bool_value().unwrap(),
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

impl From<&Vec<AbstractSyntaxProperty>> for ColouredLabelProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
                AbstractSyntaxPropertyType::Colour => to.colour = property.value().get_colour_value().unwrap().into(),
                _ => {}
            }
        }
        to
    }
}

#[derive(Default)]
pub struct SelectableLabelProperties {
    pub text: String,
    pub selected: bool,
    pub on_selected: Function
}

impl From<&Vec<AbstractSyntaxProperty>> for SelectableLabelProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
                AbstractSyntaxPropertyType::Selected => to.selected = property.value().get_bool_value().unwrap(),
                AbstractSyntaxPropertyType::OnSelect => to.on_selected = property.value().get_function_value().unwrap(),
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

impl From<&Vec<AbstractSyntaxProperty>> for MonospaceProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
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

impl From<&Vec<AbstractSyntaxProperty>> for CodeProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
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

impl From<&Vec<AbstractSyntaxProperty>> for HeadingProperties {
    fn from(from: &Vec<AbstractSyntaxProperty>) -> Self {
        let mut to = Self::default();
        for property in from {
            match property.property_type() {
                AbstractSyntaxPropertyType::Text => to.text = property.value().get_string_value().unwrap(),
                _ => {}
            }
        }
        to
    }
}
