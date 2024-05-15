use mono_event_derive::listeners_capacity;

pub use mono_event_derive::{event, listen};

use std::io::Result;

pub trait EventListener<E, T> {
    fn __listen(event: &mut E) -> Result<()>;
}

listeners_capacity!(1000);
