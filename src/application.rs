use crate::prelude::*;

#[derive(Debug)]
pub enum RuxError {
    FileMonitoringError(FileMonitorError),
    FileMonitoringWatchError(FileMonitorWatchError),
    SourceReadingError(SourceReaderError),
    RendererError(RendererError)
}

impl From<SourceReaderError> for RuxError {
    fn from(from: SourceReaderError) -> Self {
        RuxError::SourceReadingError(from)
    }
}


impl From<FileMonitorError> for RuxError {
    fn from(from: FileMonitorError) -> Self {
        RuxError::FileMonitoringError(from)
    }
}

impl From<FileMonitorWatchError> for RuxError {
    fn from(from: FileMonitorWatchError) -> Self {
        RuxError::FileMonitoringWatchError(from)
    }
}

impl From<RendererError> for RuxError {
    fn from(from: RendererError) -> Self {
        RuxError::RendererError(from)
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

    pub fn build(self) -> Result<ApplicationRunner, RuxError> {
        let event_loop = create_system_event_loop();
        let mut data_context = create_data_context();
        (self.on_context)(&mut data_context);
        
        Ok(ApplicationRunner::new(
            data_context,
            SourceFiles::new(self.relative_rux_folder_path, self.file_monitor_poll)?,
            AbstractSyntax::default(),
            Renderer::new(&event_loop)?,
            event_loop
        ))
    }
}

pub struct ApplicationRunner {
    data_context: DataContext,
    source_files: SourceFiles,
    ast: AbstractSyntax,
    renderer: Renderer,
    event_loop: SystemEventLoop
}

impl ApplicationRunner {
    fn new(data_context: DataContext, source_files: SourceFiles, ast: AbstractSyntax, renderer: Renderer, event_loop: SystemEventLoop) -> Self {
        Self {
            data_context,
            source_files, 
            ast,
            renderer,
            event_loop
        }
    }

    pub fn run(&mut self) -> Result<(), RuxError> {
        loop {
            if !self.run_loop()? {
                return Ok(());
            }
        }
    }

    fn run_loop(&mut self) -> Result<bool, RuxError> {
        if !self.process_events() {
            return Ok(false);
        }
        self.execute()?;
        return Ok(true);
    }

    fn process_events(&mut self) -> bool {
        self.event_loop.run(&mut self.renderer)
    }

    fn execute(&mut self) -> Result<(), RuxError> {
        let changes = self.source_files.process()?;
        self.ast.build(&changes, &mut self.source_files, &mut self.data_context);
        self.renderer.render(&mut self.data_context, &mut self.ast);
        Ok(())
    }
}
