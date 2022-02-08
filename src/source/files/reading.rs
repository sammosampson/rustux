use crate::prelude::*;
use std::fs;

pub fn create_file_paths(relative_folder_path: &'static str) -> FilePaths {
    FilePaths::new(relative_folder_path)
}
#[derive(Clone, Copy)]
pub struct FilePaths {
    relative_folder_path: &'static str
}

impl FilePaths {
    pub fn new(relative_folder_path: &'static str) -> Self {
        FilePaths {
            relative_folder_path
        }
    }

    pub fn get_absolute_folder_path(&self) -> Result<PathBuf, FilePathError> {
        let path = std::env::var("CARGO_MANIFEST_DIR").map_err(|_|FilePathError::ManifestDirectoryEnvironmentVariableNotSet)?;
        info!("manifest path is {:?}", path);
        Ok(PathBuf::from(path).join(self.relative_folder_path))
    }
}

impl Default for FilePaths {
    fn default() -> Self {
        Self {
            relative_folder_path: ""
        }
    }
 }

#[derive(Debug)]
pub enum Error {
    FailedToReadRuxFile(std::io::Error),
    FilePathError(FilePathError)
}

impl From<FilePathError> for Error {
    fn from(error: FilePathError) -> Error {
        Error::FilePathError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::FailedToReadRuxFile(error)
    }
}

pub fn create_source_file_reader() -> FileSourceReader {
    FileSourceReader
}

pub struct FileSourceReader;

impl SourceReader for FileSourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError> {
        Ok(fs::read_to_string(location.to_path_buf()).map_err(|_|SourceReaderError::ErrorReadingSource)?)
    }

    fn get_relative_source_location(&self, from: &SourceLocation, relative_location: &str) -> Result<SourceLocation, SourceLocationError> {
        from.to_path_buf()
            .parent().unwrap()
            .join(relative_location)
            .to_canonicalised_source_location()
        
    }
}