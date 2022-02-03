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
   
    pub fn render(&mut self, context: &mut DataContext, ast: &AbstractSyntaxGraph, display: &Display, frame: &mut Frame) -> bool {
        if let Some(root) = ast.get_root() {
            return self.render_root(context, ast, root, display, frame);
        }
        false     
    }

    pub fn render_root(&mut self, context: &mut DataContext, ast: &AbstractSyntaxGraph, root: &AbstractSyntaxGraphNode, display: &Display, frame: &mut Frame) -> bool {
        self.begin_frame(display);
        self.set_visuals();
        self.render_top_levels(context, ast, ast.get_children(root));
        self.end_frame_and_paint(display, frame)
    }

    fn render_top_levels(&self, context: &mut DataContext, ast: &AbstractSyntaxGraph, children: Vec<&AbstractSyntaxGraphNode>) {
        for child in children {
            self.render_top_level(context, ast, child)
        }
    }

    fn render_top_level(&self, context: &mut DataContext, ast: &AbstractSyntaxGraph, node: &AbstractSyntaxGraphNode) {
        match node.node_type() {
            AbstractSyntaxControlType::CentralPanel =>
                self.render_central_panel(| ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxControlType::TopPanel =>
                self.render_top_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxControlType::BottomPanel =>
                self.render_bottom_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxControlType::LeftSidebar =>
                self.render_left_side_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            AbstractSyntaxControlType::RightSidebar =>
                self.render_right_side_panel(node.properties().into(), | ui | self.render_children(ui, context, ast, node)),
            _ => {}
        }
    }

    fn render_children(&self, ui: &mut egui::Ui, context: &mut DataContext, ast: &AbstractSyntaxGraph, parent: &AbstractSyntaxGraphNode) {
        for child in ast.get_children(parent) {
            self.render_child(ui, context, ast, child)
        }
    }

    fn render_child(&self, ui: &mut egui::Ui, context: &mut DataContext, ast: &AbstractSyntaxGraph, child: &AbstractSyntaxGraphNode) {
        match child.node_type() {
            AbstractSyntaxControlType::Container => 
                self.render_children(ui, context, ast, child),
            AbstractSyntaxControlType::ScrollArea => 
                self.render_scroll_area(ui, child.properties().into(), | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxControlType::Separator => 
                self.render_separator(ui),
            AbstractSyntaxControlType::Horizontal => 
                self.render_horizontal(ui, | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxControlType::Vertical => 
                self.render_vertical(ui, | ui | self.render_children(ui, context, ast, child)),
            AbstractSyntaxControlType::Label => 
                self.render_label(ui, child.properties().into()),
            AbstractSyntaxControlType::ColouredLabel => 
                self.render_coloured_label(ui, child.properties().into()),
            AbstractSyntaxControlType::SelectableLabel => 
                self.render_selectable_label(ui, context, child.properties().into()),
            AbstractSyntaxControlType::Heading => 
                self.render_heading(ui, child.properties().into()),
            AbstractSyntaxControlType::Monospace => 
                self.render_monospace(ui, child.properties().into()),
            AbstractSyntaxControlType::Code => 
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