#![feature(macro_metavar_expr)]
use mono_event_derive::expand_listener_structs;

pub use mono_event_derive::{
    event, high_priority, highest_priority, listen, low_priority, lowest_priority, normal_priority,
};

use std::io::Result;

expand_listener_structs!();

pub trait EventListener<E, T> {
    fn __listen(event: &mut E) -> Result<()>;
}
