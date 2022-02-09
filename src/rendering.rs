use crate::prelude::*;

pub struct Renderer {
    graph_renderer: AbstractSyntaxGraphRenderer,
    screen_renderer: ScreenRenderer
}

impl Renderer {
    pub fn new(event_loop: &SystemEventLoop) -> Result<Self, RuxError> {
        let screen_renderer = create_screen_renderer(event_loop)?;
        let graph_renderer = create_graph_renderer(&screen_renderer.display);
        
        Ok(Self {
            graph_renderer,
            screen_renderer
        })
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.graph_renderer.process_event(event);
    }

    pub fn render(&mut self, context: &mut DataContext, ast: &mut AbstractSyntax) {
        let mut target = self.start_rendering();
        self.render_gui(context, ast, &mut target);   
        complete_render(target);
    }

    fn start_rendering(&mut self) -> Frame {
        self.screen_renderer.start_render()
    }

    fn render_gui(&mut self, context: &mut DataContext, ast: &mut AbstractSyntax, target: &mut Frame) {
        if self.graph_renderer.render(context, ast.graph(), &self.screen_renderer.display, target) {
            self.screen_renderer.display.gl_window().window().request_redraw();
        }
    }
}

fn complete_render(target: Frame) {
    complete_screen_render(target)
        .expect("Could not complete render");
}

fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, RuxError> {
    Ok(ScreenRenderer::new(&event_loop.get_loop())?)
}