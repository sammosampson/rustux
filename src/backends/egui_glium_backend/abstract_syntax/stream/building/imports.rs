use crate::prelude::*;

#[derive(Default)]
pub struct SourceImports {
    current_name: Option<String>,
    current_path: Option<String>,
    lookup: HashMap<String, String>
}

impl SourceImports {
    pub fn push_name(&mut self, property_value: &SourceTokenPropertyValue) -> Result<(), AbstractSyntaxTokenError> {
        match property_value {
            SourceTokenPropertyValue::String(name) => {
                self.current_name = Some(name.clone());
                self.push_current_path_if_possible();
            },
            _ => return Err(AbstractSyntaxTokenError::UnknownPropertyValue("name".to_string()))
        }
        Ok(())
    }

    pub fn push_path(&mut self, property_value: &SourceTokenPropertyValue) -> Result<(), AbstractSyntaxTokenError> {
        match property_value {
            SourceTokenPropertyValue::String(path) => {
                self.current_path = Some(path.clone());
                self.push_current_path_if_possible();
            },
            _ => return Err(AbstractSyntaxTokenError::UnknownPropertyValue("path".to_string()))
        }
        Ok(())
    }

    fn push_current_path_if_possible(&mut self) {
        if let Some(name) =  &self.current_name {
            if let Some(path) =  &self.current_path {
                self.lookup.insert(name.clone(), path.clone());
            }    
        }
    }

    pub fn get_path(&self, control_name: &str) -> Option<&String> {
        self.lookup.get(control_name)
    }
}