use crate::prelude::*;

#[system(simple)]
#[read_component(SourceFile)]
#[write_component(SourceFileParsed)]
#[write_component(SourceFileChange)]
#[write_component(SourceFileCreation)]
#[write_component(SourceFileRemoval)]
pub fn source_file_monitoring(
    command_buffer: &mut CommandBuffer,
    #[resource] monitor: &FileSystemFileMonitor,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup
) {
    
    match monitor.try_get_file_changed() {
        Ok(event) => match event {
            FileMonitorFileChange::Modify(location) => {
                modify_source(location, command_buffer, source_entity_lookup);
            },
            FileMonitorFileChange::Delete(location) => {
                delete_source(location, command_buffer, source_entity_lookup);
            },
            FileMonitorFileChange::Create(location) => {
                create_source(location, command_buffer, source_entity_lookup, source_location_lookup); 
            }
        },
        Err(_) => {

        } // handle error better?
    }
}
