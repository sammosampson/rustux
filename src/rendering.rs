
pub use glium:: {
    Frame,
    Surface,
    backend::glutin::Display,
    backend::glutin::DisplayCreationError,
    glutin:: {
        event::*,
        ContextBuilder,
        event_loop::*,
        window::*,
        platform::run_return::EventLoopExtRunReturn,
        event_loop::EventLoop,
        event::Event
    }
};

use egui_glium::*;

use crate::prelude::*;

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    BufferSwapError
}

pub struct ScreenRenderer {
    pub display: Display
}

impl ScreenRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display_for_renderer(event_loop)?;
        Ok(Self {
            display
        })
    }

    pub fn start_render(&mut self) -> Frame {
        let mut target = self.create_draw_target();        
        clear_target_color_and_depth(&mut target);
        target
    }

    fn create_draw_target(&self) -> Frame {
        self.display.draw()
    }
}

fn create_display_for_renderer(event_loop: &EventLoop<()>) -> Result<Display, RendererError> {
    Ok(create_display(event_loop).map_err(|_|RendererError::FailedToDisplayWindow)?)
}

fn clear_target_color_and_depth(target: &mut Frame) {
    target.clear_color_and_depth((0.3, 0.3, 0.5, 1.0), 1.0);
}

pub fn complete_screen_render(target: Frame) -> Result<(), RendererError> {
    Ok(target.finish().map_err(|_|RendererError::BufferSwapError)?)
}

pub fn create_display(event_loop: &EventLoop<()>) -> Result<Display, DisplayCreationError> {
    Display::new(WindowBuilder::new().with_maximized(true), ContextBuilder::new().with_depth_buffer(24), event_loop)
}

pub fn create_egui_renderer(display: &Display) -> EguiRenderer {
    EguiRenderer::new(display)
}

pub struct EguiRenderer {
    egui: EguiGlium
}

impl EguiRenderer {
    pub fn new(display: &Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }

    pub fn set_visuals(&mut self) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
    }

    pub fn begin_frame(&mut self, display: &Display) {
        self.egui.begin_frame(display);
    }

    pub fn end_frame_and_paint(&mut self, display: &Display, target: &mut Frame) -> bool {
        let (needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);
        needs_repaint
    }

    pub fn render_left_side_panel(
        &self, 
        name: &str
    ) {
        egui::SidePanel::left(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui | self.add_contents(ui));
    }

    pub fn render_right_side_panel(
        &self, 
        name: &str
    ) {
        egui::SidePanel::right(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui | self.add_contents(ui));
    }

    fn add_contents(&self, ui: &mut egui::Ui) {

    }
}

pub struct AbstractSystaxTreeRenderer<'a> {
    display: &'a Display,
    frame: &'a mut Frame,
    renderer: &'a mut EguiRenderer
}

impl<'a> AbstractSystaxTreeRenderer<'a> {
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
            self.render_top_level(&child.node_type())
        }
    }

    fn render_top_level(&self, node_type: &AbstractSyntaxTokenType) {
        match node_type {
            AbstractSyntaxTokenType::LeftSidebar => 
                self.renderer.render_left_side_panel("test"),
            AbstractSyntaxTokenType::RightSidebar => 
                self.renderer.render_right_side_panel("test"),
            _ => {}
        }
    }
}
