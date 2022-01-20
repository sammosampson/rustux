mod screen;
mod egui_render;

pub use screen::*;
pub use egui_render::*;

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

