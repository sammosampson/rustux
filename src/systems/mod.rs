mod parsing;
mod source_build;
mod source_monitor;
mod remove_entities;
mod rendering;

use parsing::*;
use source_build::*;
use source_monitor::*;
use remove_entities::*;
use rendering::*;

pub use legion::*;
pub use legion::query::Query;
pub use legion::systems::CommandBuffer;
pub use legion::systems::Resources;
pub use legion::systems::Builder;
pub use legion::world::SubWorld;
pub use legion::storage::Component;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(recurisve_source_location_build_system())
        .add_thread_local(source_file_monitoring_system())
        .flush()
        .add_system(source_token_removal_system())
        .add_thread_local(source_parse_system())
        .flush()
        .add_thread_local(render_system())    
        .add_system(remove_entity_system())
        .build()
}