mod panels;
mod labels;
mod grouping;

pub use panels::*;
pub use labels::*;
pub use grouping::*;

use crate::prelude::*;
use egui_glium::*;


pub fn create_ast_renderer(display: &Display) -> AbstractSyntaxTreeRenderer {
    AbstractSyntaxTreeRenderer::new(display)
}

pub struct AbstractSyntaxTreeRenderer {
    egui: EguiGlium
}

impl AbstractSyntaxTreeRenderer {
    pub fn new(display: &Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }
   
    pub fn render(&mut self, ast: &AbstractSyntaxTree, display: &Display, frame: &mut Frame) -> bool {
        if let Some(root) = ast.get_root() {
            return self.render_root(ast, root, display, frame);
        }
        false     
    }

    pub fn render_root(&mut self, ast: &AbstractSyntaxTree, root: &AbstractSyntaxTreeNode, display: &Display, frame: &mut Frame) -> bool {
        self.begin_frame(display);
        self.set_visuals();
        self.render_top_levels(ast, ast.get_children(root));
        self.end_frame_and_paint(display, frame)
    }

    fn render_top_levels(&self, ast: &AbstractSyntaxTree, children: Vec<&AbstractSyntaxTreeNode>) {
        for child in children {
            self.render_top_level(ast, child)
        }
    }

    fn render_top_level(&self, ast: &AbstractSyntaxTree, node: &AbstractSyntaxTreeNode) {
        match node.node_type() {
            AbstractSyntaxTokenType::CentralPanel =>
                self.render_central_panel(| ui | self.render_children(ui, ast, node)),
            AbstractSyntaxTokenType::TopPanel =>
                self.render_top_panel(node.properties().into(), | ui | self.render_children(ui, ast, node)),
            AbstractSyntaxTokenType::BottomPanel =>
                self.render_bottom_panel(node.properties().into(), | ui | self.render_children(ui, ast, node)),
            AbstractSyntaxTokenType::LeftSidebar =>
                self.render_left_side_panel(node.properties().into(), | ui | self.render_children(ui, ast, node)),
            AbstractSyntaxTokenType::RightSidebar =>
                self.render_right_side_panel(node.properties().into(), | ui | self.render_children(ui, ast, node)),
            _ => {}
        }
    }

    fn render_children(&self, ui: &mut egui::Ui, ast: &AbstractSyntaxTree, parent: &AbstractSyntaxTreeNode) {
        for child in ast.get_children(parent) {
            self.render_child(ui, ast, child)
        }
    }

    fn render_child(&self, ui: &mut egui::Ui, ast: &AbstractSyntaxTree, child: &AbstractSyntaxTreeNode) {
        match child.node_type() {
            AbstractSyntaxTokenType::ScrollArea => 
                self.render_scroll_area(
                    get_name_property(child.properties()),
                get_vertical_size_property(child.properties()), ui, | ui | self.render_children(ui, ast, child)),
            AbstractSyntaxTokenType::Separator => 
                self.render_separator(ui),
            AbstractSyntaxTokenType::Horizontal => 
                self.render_horizontal(ui, | ui | self.render_children(ui, ast, child)),
            AbstractSyntaxTokenType::Vertical => 
                self.render_vertical(ui, | ui | self.render_children(ui, ast, child)),
            AbstractSyntaxTokenType::Label => 
                self.render_label(ui, child.properties().into()),
            AbstractSyntaxTokenType::ColouredLabel => 
                self.render_coloured_label(ui, child.properties().into()),
            AbstractSyntaxTokenType::SelectableLabel => 
                self.render_selectable_label(ui, child.properties().into()),
            AbstractSyntaxTokenType::Heading => 
                self.render_heading(ui, child.properties().into()),
            AbstractSyntaxTokenType::Monospace => 
                self.render_monospace(ui, child.properties().into()),
            AbstractSyntaxTokenType::Code => 
                self.render_code(ui, child.properties().into()),
            _ => {}
        }
    }

    fn set_visuals(&mut self) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
    }

    fn begin_frame(&mut self, display: &Display) {
        self.egui.begin_frame(display);
    }

    fn end_frame_and_paint(&mut self, display: &Display, target: &mut Frame) -> bool {
        let (needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);
        needs_repaint
    }
}

fn get_name_property(properties: &[AbstractSyntaxTokenProperty]) -> &str {
    for property in properties {
        match property {
            AbstractSyntaxTokenProperty::Id(value) => return value,
            _ => {}
        }
    }

    return "";
}

fn get_text_property(properties: &[AbstractSyntaxTokenProperty]) -> &str {
    for property in properties {
        match property {
            AbstractSyntaxTokenProperty::Text(value) => return value,
            _ => {}
        }
    }

    return "";
}

fn get_selected_property(properties: &[AbstractSyntaxTokenProperty]) -> bool {
    for property in properties {
        match property {
            AbstractSyntaxTokenProperty::Selected(value) => return *value,
            _ => {}
        }
    }

    return false;
}

fn get_vertical_size_property(properties: &[AbstractSyntaxTokenProperty]) -> VerticalSize {
    for property in properties {
        match property {
            AbstractSyntaxTokenProperty::VerticallySized(value) => return *value,
            _ => {}
        }
    }

    return VerticalSize::Auto;
}