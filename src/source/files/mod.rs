mod file_reading;
mod folder_walking;
mod monitoring;
mod source_files;

pub use monitoring::*;
pub use source_files::*;
pub use file_reading::*;
pub use folder_walking::*;

pub use std::path::PathBuf;


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