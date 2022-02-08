use crate::prelude::*;

#[derive(Default)]
pub struct State {
    items: HashMap<usize, Box<dyn std::any::Any>>
}

impl State {
    pub fn process<T:std::any::Any + Default>(&mut self, id: usize, processor: Box<dyn FnOnce(&T) -> T>) {
        let processed_state = if let Some(item) = self.get(id) {
            processor(item.downcast_ref::<T>().unwrap())
        } else {
            processor(&T::default())
        };

        self.set(id, processed_state)
    }

    fn set<T:std::any::Any + Default>(&mut self, id: usize, to_set: T) {
        self.items.insert(id, Box::new(to_set));
    }

    pub fn get_local<T:std::any::Any + Default>(&mut self) -> &T {
        let id = self.get_local_state_id();

        if !self.items.contains_key(&id) {
            self.set(id, T::default());
        }
        self.get(id).unwrap().downcast_ref::<T>().unwrap()
    }

    fn get(&self, id: usize) -> Option<&Box<dyn std::any::Any>> {
        self.items.get(&id)
    }

    fn get_local_state_id(&self) -> usize {
        1
    }
}