use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataArrayId(usize);


#[derive(Default)]
pub struct DataArrays(Vec<Box<dyn DataArray>>);

impl DataArrays {
    pub fn add_string_array(&mut self, items: Vec<String>) -> DataArrayId {
        self.add(StringDataArray::from(items))
    }

    pub fn add(&mut self, item: impl DataArray + 'static) -> DataArrayId {
        let id = DataArrayId(self.0.len());
        self.0.insert(id.0, Box::new(item));
        id
    }

    pub fn get(&self, id: DataArrayId) -> Option<&Box<dyn DataArray>> {
        self.0.get(id.0)
    }

    pub fn get_mut(&mut self, id: DataArrayId) -> Option<&mut Box<dyn DataArray>> {
        self.0.get_mut(id.0)
    }    
}

pub trait DataArray {
    fn len(&self) -> usize;
    fn get_array_item_value(&self, position: usize, variable: &VariablePath) -> Option<AbstractSyntaxPropertyValue>;
}

pub struct StringDataArray(Vec<String>);

impl DataArray for StringDataArray {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_array_item_value(&self, position: usize, _variable: &VariablePath) -> Option<AbstractSyntaxPropertyValue> {
        if let Some(value) = self.0.get(position) {
            return Some(AbstractSyntaxPropertyValue::String(value.clone()));
        }
        None
    }
}

impl From<Vec<String>> for StringDataArray {
    fn from(from: Vec<String>) -> Self {
        Self(from)
    }
}