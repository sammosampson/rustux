mod application;
mod systems;
mod events;
mod source;
mod backends;
mod state;
mod examples;

mod prelude {
    pub use log::{debug,info}; 
    pub use std::error::Error; 
    pub use std::fmt::{ Formatter }; 
    pub use std::ops::*;
    pub use std::collections::*;
    pub use std::marker::PhantomData;
    pub use std::iter::Enumerate;
    pub use std::str::Chars;
    pub use core::fmt::Debug;
    pub use core::time::Duration;
    pub use crate::application::*;
    pub use crate::systems::*;
    pub use crate::events::*;
    pub use crate::source::*;
    pub use crate::backends::*;
    pub use crate::state::*;
}

use crate::prelude::*;

fn main() {
    Application::default()
        .use_logging()
        .with_file_path("examples/assets/first")
        .with_file_monitor_poll(Duration::from_secs(1))
        .with_context(|ctx| examples::first::register_actions(ctx))
        .build()
        .expect("Build error")
        .run();
}
