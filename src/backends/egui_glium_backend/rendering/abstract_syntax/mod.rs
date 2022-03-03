mod panels;
mod labels;
mod grouping;

pub use panels::*;
pub use labels::*;
pub use grouping::*;

use crate::prelude::*;
use egui_glium::*;


pub fn create_graph_renderer(display: &Display) -> AbstractSyntaxGraphRenderer {
    AbstractSyntaxGraphRenderer::new(display)
}

pub struct AbstractSyntaxGraphRenderer {
    egui: EguiGlium
}

impl AbstractSyntaxGraphRenderer {
    pub fn new(display: &Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }
   
    pub fn render(&mut self, context: &mut DataContext, graph: &AbstractSyntaxGraph, display: &Display, frame: &mut Frame) -> bool {
        if let Some(root) = graph.get_root() {
            return self.render_root(context, graph, root, display, frame);
        }
        false     
    }

    pub fn render_root(&mut self, context: &mut DataContext, graph: &AbstractSyntaxGraph, root: &AbstractSyntaxGraphNode, display: &Display, frame: &mut Frame) -> bool {
        self.begin_frame(display);
        self.set_visuals();
        push_scope(context, root);
        self.render_top_levels(context, graph, graph.get_children(root));
        pop_scope(context);
        self.end_frame_and_paint(display, frame)
    }

    fn render_top_levels(&self, context: &mut DataContext, graph: &AbstractSyntaxGraph, children: Vec<&AbstractSyntaxGraphNode>) {
        for child in children {
            self.render_top_level(context, graph, child)
        }
    }

    fn render_top_level(&self, context: &mut DataContext, graph: &AbstractSyntaxGraph, node: &AbstractSyntaxGraphNode) {
        match node.node_type() {
            AbstractSyntaxControlType::CentralPanel =>
                self.render_central_panel(| ui | self.render_children(ui, context, graph, node)),
            AbstractSyntaxControlType::TopPanel =>
                self.render_top_panel(node.properties().into(), | ui | self.render_children(ui, context, graph, node)),
            AbstractSyntaxControlType::BottomPanel =>
                self.render_bottom_panel(node.properties().into(), | ui | self.render_children(ui, context, graph, node)),
            AbstractSyntaxControlType::LeftSidebar =>
                self.render_left_side_panel(node.properties().into(), | ui | self.render_children(ui, context, graph, node)),
            AbstractSyntaxControlType::RightSidebar =>
                self.render_right_side_panel(node.properties().into(), | ui | self.render_children(ui, context, graph, node)),
            _ => {}
        }
    }

    fn render_child(&self, ui: &mut egui::Ui, context: &mut DataContext, graph: &AbstractSyntaxGraph, child: &AbstractSyntaxGraphNode) {
        match child.node_type() {
            AbstractSyntaxControlType::Scope => 
                self.set_scope(ui, context, graph, child),
            AbstractSyntaxControlType::Container => 
                self.render_children(ui, context, graph, child),
            AbstractSyntaxControlType::ScrollArea => 
                self.render_scroll_area(ui, child.properties().into(), | ui | self.render_children(ui, context, graph, child)),
            AbstractSyntaxControlType::Separator => 
                self.render_separator(ui),
            AbstractSyntaxControlType::Horizontal => 
                self.render_horizontal(ui, | ui | self.render_children(ui, context, graph, child)),
            AbstractSyntaxControlType::Vertical => 
                self.render_vertical(ui, | ui | self.render_children(ui, context, graph, child)),
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

    fn render_children(&self, ui: &mut egui::Ui, context: &mut DataContext, graph: &AbstractSyntaxGraph, parent: &AbstractSyntaxGraphNode) {
        for child in graph.get_children(parent) {
            self.render_child(ui, context, graph, child)
        }
    }

    fn set_scope(&self, ui: &mut egui::Ui, context: &mut DataContext, graph: &AbstractSyntaxGraph, node: &AbstractSyntaxGraphNode) {
        push_scope(context, node);
        self.render_children(ui, context, graph, node);
        pop_scope(context);
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

fn push_scope(context: &mut DataContext, node: &AbstractSyntaxGraphNode) {
    context.scopes_mut().push(node.id())
}

fn pop_scope(context: &mut DataContext) {
    context.scopes_mut().pop();
}