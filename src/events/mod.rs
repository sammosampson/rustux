mod loops;
mod channel;
mod producer; 

pub use shrev::{
    EventChannel, ReaderId, EventIterator
};
pub use loops::*;
pub use channel::*;
pub use producer::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum SystemEvent {
    CloseRequested,
}