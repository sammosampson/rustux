use crate::prelude::*;

pub fn create_system_event_loop() -> SystemEventLoop {
    SystemEventLoop {
        inner: EventLoop::new()
    }
}

pub struct SystemEventLoop {
    inner: EventLoop<()>
}

impl SystemEventLoop {
    pub fn get_loop(&self) -> &EventLoop<()> {
        &self.inner
    }

    pub fn run(&mut self, renderer: &mut Renderer) -> bool {
        let mut close = false;
        self.inner.run_return(|event, _, flow| {
            match event {
                Event::WindowEvent { window_id: _, event} => {
                    renderer.process_event(&event);
                    match event {
                        WindowEvent::CloseRequested => close = true,
                        _ => {} 
                    }
                    
                },
                _ => {}
            }
            *flow = glium::glutin::event_loop::ControlFlow::Exit;
        });

        return !close;
    }
}
