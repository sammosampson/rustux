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
    on_context: Box<dyn FnOnce(&mut StateContext) -> ()>
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

    pub fn with_context(mut self, on_context: impl FnOnce(&mut StateContext) -> () + 'static) -> Self {
        self.on_context = Box::new(on_context);
        self
    }

    pub fn build(self) -> Result<ApplicationRunner, RustuxError> {
        let mut resources = Resources::default();

        let file_paths = create_file_paths(self.relative_rux_folder_path);
        let event_loop = create_system_event_loop();
        let screen_renderer = create_screen_renderer(&event_loop)?;
        let egui_renderer = create_ast_renderer(&screen_renderer.display);
        let mut state_context= create_state_context();
        (self.on_context)(&mut state_context);
    
        resources.insert(create_system_event_channel());
        resources.insert(file_paths);
        resources.insert(screen_renderer);
        resources.insert(egui_renderer);
        resources.insert(create_file_system_source_location_walker());
        resources.insert(monitor_files(file_paths, self.file_monitor_poll)?);
        resources.insert(create_source_file_reader());
        resources.insert(create_source_entity_lookup());
        resources.insert(create_abstract_syntax_token_stream_lookup());
        resources.insert(create_source_location_lookup());
        resources.insert(create_system_event_producer());
        resources.insert(state_context);
        Ok(ApplicationRunner::new(event_loop, build_schedule(), resources))
    }
}

pub struct ApplicationRunner {
    world: World,
    resources: Resources,
    schedule: Schedule,
    event_loop: SystemEventLoop
}

impl ApplicationRunner {
    fn new(event_loop: SystemEventLoop, schedule: Schedule, resources: Resources) -> Self {
        Self {
            world: World::default(),
            schedule,
            resources,
            event_loop
        }
    }

    pub fn run(&mut self) {
        loop {
            self.run_loop();
        }
    }

    fn run_loop(&mut self) {
        self.process_events();
        self.execute_schedule();
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<SystemEventChannel>().unwrap();
        let mut editor_renderer = &mut self.resources.get_mut::<AbstractSyntaxTreeRenderer>().unwrap();
        
        self.event_loop.run(
            &mut event_producer,
            &mut event_channel,
            &mut editor_renderer,
        );
    }

    fn execute_schedule(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, RustuxError> {
    Ok(ScreenRenderer::new(&event_loop.get_loop())?)
}
