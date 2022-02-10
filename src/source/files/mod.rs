mod monitoring;
mod location;
mod reading;
mod changes;

pub use monitoring::*;
pub use location::*;
pub use reading::*;
pub use changes::*;

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
    NoFileChanges,
    SourceLocationError(SourceLocationError)
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

impl From<SourceLocationError> for FileMonitorWatchError {
    fn from(error: SourceLocationError) -> FileMonitorWatchError {
        FileMonitorWatchError::SourceLocationError(error)
    }
}

pub struct SourceFiles {
    file_paths: FilePaths,
    source_reader: FileSourceReader,
    source_tokens_lookup: SourceLookup,
    monitor: FileSystemFileMonitor,
    initially_parsed: bool
}

impl SourceFiles {
    pub fn new(relative_rux_folder_path: &'static str, file_monitor_poll: Duration) -> Result<Self, RuxError> {
        let file_paths = create_file_paths(relative_rux_folder_path);
        
        let source_files = SourceFiles { 
            file_paths,
            source_reader: create_source_file_reader(),
            source_tokens_lookup: create_source_lookup(),
            monitor: monitor_files(file_paths, file_monitor_poll)?,
            initially_parsed: false
        };
        
        Ok(source_files)
    }

    pub fn lookup(&self, location: &SourceLocation) -> Option<&String> {    
        self.source_tokens_lookup.get(location) 
    }

    pub fn process(&mut self) -> Result<SourceChanges, RuxError> {
        if !self.initially_parsed {
            self.parse_source_locations_recurisvely()
        } else {
            self.process_changes()
        }
    }

    fn parse_source_locations_recurisvely(&mut self) -> Result<SourceChanges, RuxError> {   
        let mut changes = create_source_changes();
        let source_location_walker = create_file_system_source_location_walker();

        for location in source_location_walker.walk(&mut self.file_paths).unwrap() {
            self.parse_source(location.clone())?;                   
            changes.push(location);
        }    
        
        self.initially_parsed = true;

        Ok(changes)
    }

    fn process_changes(&mut self) -> Result<SourceChanges, RuxError> {
        match self.monitor.try_get_file_changed() {
            Ok(event) => match event {
                FileMonitorFileChange::Modify(location) => {
                    self.parse_source(location.clone())?;
                    Ok(SourceChanges::from(location))
                },
                FileMonitorFileChange::Delete(location) => {
                    self.delete_source(location.clone());
                    Ok(SourceChanges::from(location))
                },
                FileMonitorFileChange::Create(location) => {
                    self.parse_source(location.clone())?;
                    Ok(SourceChanges::from(location)) 
                }
            },
            Err(FileMonitorWatchError::NoFileChanges) => Ok(create_source_changes()),
            Err(error) => Err(error.into())
        }
    }

    fn delete_source(&mut self, location: SourceLocation) {
        self.source_tokens_lookup.remove(&location);     
    }

    fn parse_source(&mut self, location: SourceLocation) -> Result<(), SourceReaderError> {    
        let source_text = self.source_reader.read_source_at_location(&location)?;
        debug!("Source is now {:?} chars", source_text.len());
        self.source_tokens_lookup.insert(location, source_text); 
        Ok(())
    }
}

