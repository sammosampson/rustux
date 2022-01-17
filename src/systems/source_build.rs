use crate::prelude::*;

#[system(simple)]
#[write_component(SourceFile)]
pub fn recurisve_source_location_build(
    command_buffer: &mut CommandBuffer,
    world: &mut SubWorld,
    #[resource] file_paths: &mut FilePaths,
    #[resource] source_location_walker: &mut FileSystemSourceLocationWalker,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup
) {    
    let source_files: Vec::<&SourceFile> = <&SourceFile>::query().iter(world).collect();

    if source_files.len() > 0 {
        return;
    }

    for location in source_location_walker.walk(file_paths).unwrap() {
        read_source(location, command_buffer, source_entity_lookup, source_location_lookup);                   
    }    
}

