use crate::prelude::*;

pub fn create_system_event_channel() -> SystemEventChannel {
    SystemEventChannel::new()
}

pub struct SystemEventChannel {
    channel: EventChannel::<SystemEvent>,
    reader_id: ReaderId<SystemEvent>
}

impl SystemEventChannel {
    pub fn new() -> Self {
        let mut channel = EventChannel::<SystemEvent>::new();
        let reader_id = channel.register_reader();
        Self {
            channel,
            reader_id
        }
    }

    pub fn drain(&mut self, events: &mut Vec<SystemEvent>) {
        self.channel.drain_vec_write(events);
    }

    pub fn read_events(&mut self) -> EventIterator<SystemEvent> {
        self.channel.read(&mut self.reader_id)
    }
}