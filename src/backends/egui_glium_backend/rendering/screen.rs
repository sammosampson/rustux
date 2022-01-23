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