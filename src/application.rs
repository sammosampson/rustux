use crate::prelude::*;

#[derive(Debug)]
pub enum RustuxError {
    FileMonitoringError(FileMonitorError),
    RendererError(RendererError)
}

impl From<FileMonitorError> for RustuxError {
    fn from(from: FileMonitorError) -> Self {
        RustuxError::FileMonitoringError(from)
    }
}

impl From<RendererError> for RustuxError {
    fn from(from: RendererError) -> Self {
        RustuxError::RendererError(from)
    }
}

pub struct Application {
    relative_rux_folder_path: &'static str,
    file_monitor_poll: Duration,
    on_context: Box<dyn FnOnce(&mut DataContext) -> ()>
}

impl Default for Application {
    fn default() -> Self {
        Self {
            relative_rux_folder_path: "",
            file_monitor_poll: Duration::default(),
            on_context: Box::new(|_| {})
        }
    }
}

impl Application {
    pub fn use_logging(self) -> Self {
        pretty_env_logger::init();
        self
    }

    pub fn with_file_path(mut self, path: &'static str) -> Self {
        self.relative_rux_folder_path = path;
        self
    }

    pub fn with_file_monitor_poll(mut self, poll: Duration) -> Self {
        self.file_monitor_poll = poll;
        self
    }

    pub fn with_context(mut self, on_context: impl FnOnce(&mut DataContext) -> () + 'static) -> Self {
        self.on_context = Box::new(on_context);
        self
    }

    pub fn build(self) -> Result<ApplicationRunner, RustuxError> {
        let event_loop = create_system_event_loop();
        
        Ok(ApplicationRunner::new(
            SourceFiles::new(self.relative_rux_folder_path, self.file_monitor_poll)?,
            Renderer::new(&event_loop, self.on_context)?,
            event_loop
        ))
    }
}

pub struct ApplicationRunner {
    source_files: SourceFiles,
    renderer: Renderer,
    event_loop: SystemEventLoop
}

impl ApplicationRunner {
    fn new(source_files: SourceFiles, renderer: Renderer, event_loop: SystemEventLoop) -> Self {
        Self {
            source_files, 
            renderer,
            event_loop
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.run_loop() {
                return
            }
        }
    }

    fn run_loop(&mut self) -> bool {
        if !self.process_events() {
            return false;
        }
        self.execute();
        return true;
    }

    fn process_events(&mut self) -> bool {
        self.event_loop.run(&mut self.renderer)
    }

    fn execute(&mut self) {
        self.source_files.process();
        let graph = self.renderer.build_graph(&mut self.source_files);
        self.renderer.render(graph);
    }
}
