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
   
    pub fn render(&mut self, context: &mut StateContext, ast: &AbstractSyntaxGraph, display: &Display, frame: &mut Frame) -> bool {
        if let Some(root) = ast.get_root() {
            return self.render_root(context, ast, root, display, frame);
        }
        false     
    }

    pub fn render_root(&mut self, context: &mut StateContext, ast: &AbstractSyntaxGraph, root: &AbstractSyntaxGraphNode, display: &Display, frame: &mut Frame) -> bool {
        self.begin_frame(display);
        self.set_visuals();
        self.render_top_levels(context, ast, ast.get_children(root));
        self.end_frame_and_paint(display, frame)
    }

    fn render_top_levels(&self, context: &mut StateContext, ast: &AbstractSyntaxGraph, children: Vec<&AbstractSyntaxGraphNode>) {
        for child in children {
            self.render_top_level(context, ast, child)
        }
    }

    fn render_top_level(&self, context: &mut StateContext, ast: &AbstractSyntaxGraph, node: &AbstractSyntaxGraphNode) {
        match node.node_type() {
            AbstractSyntaxTokenType::CentralPanel =>
                self.render_central_panel(| ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxTokenType::TopPanel =>
                self.render_top_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxTokenType::BottomPanel =>
                self.render_bottom_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxTokenType::LeftSidebar =>
                self.render_left_side_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxTokenType::RightSidebar =>
                self.render_right_side_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            _ => {}
        }
    }

    fn render_children(&self, ui: &mut egui::Ui, context: &mut StateContext, ast: &AbstractSyntaxGraph, parent: &AbstractSyntaxGraphNode) {
        for child in ast.get_children(parent) {
            self.render_child(ui, context, ast, child)
        }
    }

    fn render_child(&self, ui: &mut egui::Ui, context: &mut StateContext, ast: &AbstractSyntaxGraph, child: &AbstractSyntaxGraphNode) {
        match child.node_type() {
            AbstractSyntaxTokenType::Container => 
                self.render_children(ui, context, ast, child),
            AbstractSyntaxTokenType::ScrollArea => 
                self.render_scroll_area(ui, child.properties().into(), | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxTokenType::Separator => 
                self.render_separator(ui),
            AbstractSyntaxTokenType::Horizontal => 
                self.render_horizontal(ui, | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxTokenType::Vertical => 
                self.render_vertical(ui, | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxTokenType::Label => 
                self.render_label(ui, child.properties().into()),
            AbstractSyntaxTokenType::ColouredLabel => 
                self.render_coloured_label(ui, child.properties().into()),
            AbstractSyntaxTokenType::SelectableLabel => 
                self.render_selectable_label(ui, context, child.properties().into()),
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