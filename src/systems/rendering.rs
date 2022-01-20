use crate::prelude::*;

#[system(for_each)]
#[filter(component::<SourceFileRoot>())]
#[filter(component::<SourceFileParsed>())]
pub fn render(
    entity: &Entity,
    #[resource] abstract_syntax_token_stream_lookup: &mut AbstractSyntaxTokenStreamLookup,
    #[resource] event_producer: &mut SystemEventProducer,
    #[resource] egui_renderer: &mut EguiRenderer,
    #[resource] screen_renderer: &mut ScreenRenderer,
) {
    let ast_stream = abstract_syntax_token_stream_lookup.get_mut(entity).unwrap(); 
    let mut linker = AbstractSyntaxTokenStreamLinker::default();
    ast_stream.accept(&mut linker);

    let mut target = start_rendering(screen_renderer);
    render_gui(egui_renderer, &linker.ast(), event_producer, screen_renderer, &mut target);   
    complete_render(target);
}

fn start_rendering(screen_renderer: &mut ScreenRenderer) -> Frame {
    screen_renderer.start_render()
}

fn render_gui(
    egui_renderer: &mut EguiRenderer,
    ast: &AbstractSyntaxTree,
    event_producer: &mut SystemEventProducer,
    screen_renderer: &mut ScreenRenderer,
    target: &mut Frame
) {

    let mut renderer = AbstractSyntaxTreeRenderer::new(
        &screen_renderer.display,
        target,
        egui_renderer
    );

    if renderer.render(ast) {
        screen_renderer.display.gl_window().window().request_redraw();
    }
}

fn complete_render(target: Frame) {
    complete_screen_render(target)
        .expect("Could not complete render");
}