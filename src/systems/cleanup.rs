
use legion::*;
use legion::systems::*;
use zodiac_entities::*;


#[system(for_each)]
#[filter(component::<SourceFileInitialRead>())]
pub fn remove_source_file_initial_read(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<SourceFileInitialRead>(*entity);
}

#[system(for_each)]
#[filter(component::<SourceFileChange>())]
pub fn remove_source_file_change(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<SourceFileChange>(*entity);
}

#[system(for_each)]
#[filter(component::<SourceFileCreation>())]
pub fn remove_source_file_creation(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<SourceFileCreation>(*entity);
}

#[system(for_each)]
#[filter(component::<SourceFileRemoval>())]
pub fn remove_source_file_removal(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<SourceFileRemoval>(*entity);
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn remove_rebuild(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Rebuild>(*entity);
}

#[system(for_each)]
#[filter(component::<BuildErrorOccurrence>())]
pub fn remove_build_error(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<BuildErrorOccurrence>(*entity);
}