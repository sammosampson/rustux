use crate::prelude::*;

#[system(for_each)]
pub fn render(
    _parsed: &SourceFileParsed,
    #[resource] event_producer: &mut SystemEventProducer,
    #[resource] egui_renderer: &mut EguiRenderer,
    #[resource] ast: &mut AbstractSyntaxTree,
    #[resource] screen_renderer: &mut ScreenRenderer,
) {
    let mut target = start_rendering(screen_renderer);
    render_gui(egui_renderer, ast, event_producer, screen_renderer, &mut target);   
    complete_render(target);
}

fn start_rendering(screen_renderer: &mut ScreenRenderer) -> Frame {
    screen_renderer.start_render()
}

fn render_gui(
    egui_renderer: &mut EguiRenderer,
    ast: &mut AbstractSyntaxTree,
    event_producer: &mut SystemEventProducer,
    screen_renderer: &mut ScreenRenderer,
    target: &mut Frame
) {
    if egui_renderer.render(ast, event_producer, &screen_renderer.display, target) {
        screen_renderer.display.gl_window().window().request_redraw();
    }
}

fn complete_render(target: Frame) {
    complete_screen_render(target)
        .expect("Could not complete render");
}