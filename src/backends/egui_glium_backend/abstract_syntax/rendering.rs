
use crate::prelude::*;

pub struct AbstractSyntaxTreeRenderer<'a> {
    display: &'a Display,
    frame: &'a mut Frame,
    renderer: &'a mut EguiRenderer
}

impl<'a> AbstractSyntaxTreeRenderer<'a> {
    pub fn new(
        display: &'a Display,
        frame: &'a mut Frame,
        renderer: &'a mut EguiRenderer,
        
    ) -> Self {
        Self {
            display,
            frame,
            renderer
        }
    }
   
    pub fn render(&mut self, ast: &AbstractSyntaxTree) -> bool {
        if let Some(root) = ast.get_root() {
            return self.render_root(ast, root)
        }
        false     
    }

    pub fn render_root(&mut self, ast: &AbstractSyntaxTree, root: &AbstractSyntaxTreeNode) -> bool {
        self.renderer.begin_frame(self.display);
        self.renderer.set_visuals();
        self.render_top_levels(ast, ast.get_children(root));
        self.renderer.end_frame_and_paint(self.display, &mut self.frame)
    }

    pub fn render_top_levels(&self, ast: &AbstractSyntaxTree, children: Vec<&AbstractSyntaxTreeNode>) {
        for child in children {
            self.render_top_level(ast, child)
        }
    }

    fn render_top_level(&self, ast: &AbstractSyntaxTree, node: &AbstractSyntaxTreeNode) {
        match node.node_type() {
            AbstractSyntaxTokenType::LeftSidebar =>
                self.renderer.render_left_side_panel( get_name_property(node.properties()), | ui | self.render_children(ui, ast, node)),
            AbstractSyntaxTokenType::RightSidebar => 
                self.renderer.render_right_side_panel(get_name_property(node.properties()), | ui | self.render_children(ui, ast, node)),
            _ => {}
        }
    }

    pub fn render_children(&self, ui: &mut egui::Ui, ast: &AbstractSyntaxTree, parent: &AbstractSyntaxTreeNode) {
        for child in ast.get_children(parent) {
            self.render_child(ui, ast, child)
        }
    }

    pub fn render_child(&self, ui: &mut egui::Ui, ast: &AbstractSyntaxTree, child: &AbstractSyntaxTreeNode) {
        match child.node_type() {
            AbstractSyntaxTokenType::Horizontal => 
                self.renderer.render_horizontal(ui, | ui | self.render_children(ui, ast, child)),
            AbstractSyntaxTokenType::Vertical => 
                self.renderer.render_vertical(ui, | ui | self.render_children(ui, ast, child)),
            AbstractSyntaxTokenType::Label => 
                self.renderer.render_label(ui, get_text_property(child.properties())),
            _ => {}
        }
    }
}

fn get_name_property(properties: &[AbstractSyntaxTokenProperty]) -> &str {
    for property in properties {
        match property {
            AbstractSyntaxTokenProperty::Name(value) => return value,
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