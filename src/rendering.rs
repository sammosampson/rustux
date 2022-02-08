use crate::prelude::*;

pub struct Renderer {
    ast_renderer: AbstractSyntaxTreeRenderer,
    screen_renderer: ScreenRenderer,
    context: DataContext,
}

impl Renderer {
    pub fn new(event_loop: &SystemEventLoop, on_context: impl FnOnce(&mut DataContext) -> ()) -> Result<Self, RustuxError> {
        let screen_renderer = create_screen_renderer(event_loop)?;
        let ast_renderer = create_ast_renderer(&screen_renderer.display);
        let mut context = create_data_context();
        (on_context)(&mut context);
        
        Ok(Self {
            ast_renderer,
            screen_renderer,
            context
        })
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.ast_renderer.process_event(event);
    }

    pub fn build_graph(&mut self, source_files: &mut SourceFiles) ->  AbstractSyntaxGraph {
        let ast_stream = source_files.get_token_stream().unwrap(); 
        let mut graph_builder = AbstractSyntaxGraphBuilder::default();
        ast_stream.accept(&mut graph_builder, &mut self.context);
        graph_builder.ast()
    }

    pub fn render(&mut self, graph: AbstractSyntaxGraph) {
        let mut target = self.start_rendering();
        self.render_gui(&graph, &mut target);   
        complete_render(target);
    }

    fn start_rendering(&mut self) -> Frame {
        self.screen_renderer.start_render()
    }

    fn render_gui(&mut self, ast: &AbstractSyntaxGraph, target: &mut Frame) {
        if self.ast_renderer.render(&mut self.context, ast, &self.screen_renderer.display, target) {
            self.screen_renderer.display.gl_window().window().request_redraw();
        }
    }
}

fn complete_render(target: Frame) {
    complete_screen_render(target)
        .expect("Could not complete render");
}

fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, RustuxError> {
    Ok(ScreenRenderer::new(&event_loop.get_loop())?)
}