pub struct StateContext;

pub trait Action {
    fn path(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct ActionPointer(usize);

impl ActionPointer { 
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
