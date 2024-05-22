#![feature(min_specialization)]
#![cfg_attr(not(test), no_std)]

use mono_event_derive::expand_listeners_and_dispatching_function;

pub use mono_event_derive::{
    event, high_priority, highest_priority, listen, low_priority, lowest_priority, normal_priority,
};

expand_listeners_and_dispatching_function!();

pub trait EventListener<E, T> {
    fn __listen(event: &mut E) -> core::result::Result<(), ()>;
}
