use walkdir::WalkDir;

use crate::prelude::*;

pub trait SourceLocationWalker<T> where T: IntoIterator<Item=SourceLocation> {
    fn walk(&self, paths: &FilePaths) -> Result<T, SourceLocationWalkerError>;
}

pub fn create_file_system_source_location_walker() -> FileSystemSourceLocationWalker {
    FileSystemSourceLocationWalker::default()
}

#[derive(Default)]
pub struct FileSystemSourceLocationWalker;

#[derive(Debug)]
pub enum SourceLocationWalkerError {
    InvalidRootPath
}

impl SourceLocationWalker<FileSystemSourceLocationIterator> for FileSystemSourceLocationWalker {
    fn walk(&self, paths: &FilePaths) -> Result<FileSystemSourceLocationIterator, SourceLocationWalkerError> {
        debug!("walking tree");
        if let Ok(root_path) = paths.get_absolute_folder_path() {
            return Ok(
                FileSystemSourceLocationIterator {
                    walk_dir: WalkDir::new(root_path).into_iter()
                }
            );
        }

        Err(SourceLocationWalkerError::InvalidRootPath)        
    }
}

pub struct FileSystemSourceLocationIterator {
    walk_dir: walkdir::IntoIter
}

impl Iterator for FileSystemSourceLocationIterator {
    type Item = SourceLocation;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry= self.walk_dir.next()?;

            if let Ok(entry) = entry { 
                let path = PathBuf::from(entry.path());
                if let Some(extension) = path.extension() {
                    if extension == "rux" {
                        let location = path.to_source_location();
                        return Some(location)
                    }
                }
            } else {
                return None;  
            }
        }          
    }
}