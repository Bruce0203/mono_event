use mono_event_derive::expand_listener_structs;

pub use mono_event_derive::{event, listen};

use std::io::Result;

expand_listener_structs!();

pub trait EventListener<E, T> {
    fn __listen(event: &mut E) -> Result<()>;
}
