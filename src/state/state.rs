use crate::prelude::*;
use std::any::{ TypeId, Any };

#[derive(Default)]
pub struct State {
    items: HashMap<TypeId, Box<dyn Any>>
}

impl State {
    pub fn process<T:Any + Default>(&mut self, processor: Box<dyn FnOnce(&T) -> T>) {
        let id = TypeId::of::<T>();
        let processed_state = if let Some(item) = self.get(&id) {
            processor(item.downcast_ref::<T>().unwrap())
        } else {
            processor(&T::default())
        };

        self.set( processed_state)
    }

    fn set<T:Any + Default>(&mut self, to_set: T) {
        let id = TypeId::of::<T>();
        self.items.insert(id, Box::new(to_set));
    }

    pub fn get_local<T:Any + Default>(&mut self) -> &T {
        let id = TypeId::of::<T>();
        
        if !self.items.contains_key(&id) {
            self.set(T::default());
        }
        self.get(&id).unwrap().downcast_ref::<T>().unwrap()
    }

    fn get(&self, id: &TypeId) -> Option<&Box<dyn Any>> {
        self.items.get(id)
    }
}