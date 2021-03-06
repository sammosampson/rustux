use std::{ path::*};

use crate::prelude::*;

pub trait ToSourceLocationConversion {
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError>;
    fn to_source_location(&self) -> SourceLocation;
}

pub trait ToPathBufConversion {
    fn to_path_buf(&self) -> PathBuf;
}


#[cfg(not(target_os = "windows"))]
fn remove_canonicalization_prefix<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    PathBuf::from(path.as_ref().display().to_string())
}

#[cfg(target_os = "windows")]
fn remove_canonicalization_prefix<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let path = path.as_ref().display().to_string();
    if path.starts_with(VERBATIM_PREFIX) {
        PathBuf::from(path[VERBATIM_PREFIX.len()..].to_string())
    } else {
        PathBuf::from(path)
    }
}

impl ToSourceLocationConversion for PathBuf {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        if let Ok(path) = self.canonicalize() {
            Ok(SourceLocation { 
                location: match remove_canonicalization_prefix(path).to_str() {
                    Some(path) => Some(path.to_owned().replace("/", "\\")),
                    None => None
                }
            })
        } else {
            Err(SourceLocationError::DoesNotExist)
        }
    }

    fn to_source_location(&self) -> SourceLocation {
        SourceLocation { 
            location: Some(self.to_str().unwrap().to_owned())
        }
    }
}


impl ToSourceLocationConversion for &str {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        PathBuf::from(self).to_canonicalised_source_location()
    }

    fn to_source_location(&self) -> SourceLocation {
        PathBuf::from(self).to_source_location()
    }
}

impl ToSourceLocationConversion for String {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        PathBuf::from(self).to_canonicalised_source_location()
    }

    fn to_source_location(&self) -> SourceLocation {
        PathBuf::from(self).to_source_location()
    }
}

impl ToPathBufConversion for &SourceLocation {    
    fn to_path_buf(&self) -> std::path::PathBuf {
        if let Some(location) = &self.location {
            return std::path::PathBuf::from(location)
        }
        std::path::PathBuf::new()
    }
}