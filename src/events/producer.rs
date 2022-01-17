use crate::prelude::*;

pub fn create_system_event_producer() -> SystemEventProducer {
    SystemEventProducer::new()
}
pub struct SystemEventProducer {
    events: Vec<SystemEvent>,
}

impl SystemEventProducer {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(128),
        }
    }

    pub fn push(&mut self, to_push: SystemEvent) {
        self.events.push(to_push);
    }

    pub fn drain_to(&mut self, channel: &mut SystemEventChannel) {
        channel.drain(&mut self.events);
    }
}