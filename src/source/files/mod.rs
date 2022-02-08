mod monitoring;
mod location;
mod reading;

pub use monitoring::*;
pub use location::*;
pub use reading::*;
pub use std::path::PathBuf;

use crate::prelude::*;

#[derive(Debug)]
pub enum FileMonitorError {
    WatchError,
    FilePathError(FilePathError)
}

#[derive(Debug)]
pub enum FileMonitorWatchError {
    NoLongerMonitoring,
    NoFileChanges
}

#[derive(Debug)]
pub enum FilePathError {
    ManifestDirectoryEnvironmentVariableNotSet
}

impl From<FilePathError> for FileMonitorError {
    fn from(error: FilePathError) -> FileMonitorError {
        FileMonitorError::FilePathError(error)
    }
}

pub struct SourceFiles {
    file_paths: FilePaths,
    source_reader: FileSourceReader,
    source_tokens_lookup: AbstractSyntaxTokenStreamLookup,
    monitor: FileSystemFileMonitor,
    root_location: Option<SourceLocation>
}

impl SourceFiles {
    pub fn new(relative_rux_folder_path: &'static str, file_monitor_poll: Duration) -> Result<Self, RustuxError> {
        let file_paths = create_file_paths(relative_rux_folder_path);
        
        let mut source_files = SourceFiles { 
            file_paths,
            source_reader: create_source_file_reader(),
            source_tokens_lookup: create_abstract_syntax_token_stream_lookup(),
            monitor: monitor_files(file_paths, file_monitor_poll)?,
            root_location: None
        };
        
        source_files.build_source_locations_recurisvely();

        Ok(source_files)
    }

    fn build_source_locations_recurisvely(&mut self) {    
        let source_location_walker = create_file_system_source_location_walker();

        for location in source_location_walker.walk(&mut self.file_paths).unwrap() {
            self.read_source(location);                   
        }    
    }

    pub fn process(&mut self) {
        match self.monitor.try_get_file_changed() {
            Ok(event) => match event {
                FileMonitorFileChange::Modify(location) => {
                    self.modify_source(location);
                },
                FileMonitorFileChange::Delete(location) => {
                    self.delete_source(location);
                },
                FileMonitorFileChange::Create(location) => {
                    self.create_source(location); 
                }
            },
            Err(_) => {
                todo!("handle error better")
            } 
        }
    }

    fn read_source(&mut self, location: SourceLocation) {
        self.parse_source(location);
    }

    fn modify_source(&mut self, location: SourceLocation) {    
        self.parse_source(location);
    }
    
    fn create_source(&mut self, location: SourceLocation) {
        self.parse_source(location);
    }

    fn delete_source(&mut self, location: SourceLocation) {
        self.source_tokens_lookup.remove(&location);     
    }

    pub fn parse_source(&mut self, location: SourceLocation) {    
        let source_text = self.source_reader.read_source_at_location(&location).unwrap();
        debug!("Source is now {:?} chars", source_text.len());
    
        let source_tokenizer = SourceTokenizer::from_string(&source_text);
        let navigator = SourceTokenVisitationNavigator::from_source(source_tokenizer);
        let mut ast_build_visitor = BuildAbstractSyntaxSourceTokenVisitor::default();
        
        navigator.accept(&mut ast_build_visitor);
        let ast = ast_build_visitor.ast();
    
        if ast.contains_root() {
            self.root_location = Some(location.clone());
        }
    
        self.source_tokens_lookup.insert(location, ast); 
    }

    pub fn get_token_stream(&self) -> Option<&AbstractSyntaxTokenStream> {
        if let Some(root_location) = &self.root_location {
            return self.source_tokens_lookup.get(root_location); 
        }
        None
    }
}

