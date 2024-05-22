#![feature(min_specialization)]

use mono_event::{event, highest_priority, listen, low_priority};

#[event]
#[derive(Default)]
pub struct SayHi {
    is_listener1_invoked: bool,
    is_listener2_invoked: bool,
}

#[highest_priority]
#[listen(SayHi)]
fn print_hi(event: &mut SayHi) {
    assert!(event.is_listener1_invoked);
    event.is_listener2_invoked = true;
}

#[low_priority]
#[listen(SayHi)]
fn print_hmm(event: &mut SayHi) {
    event.is_listener1_invoked = true;
}

#[test]
fn example() {
    let mut say_hi = SayHi::default();
    say_hi.dispatch().unwrap();
    assert!(say_hi.is_listener1_invoked);
    assert!(say_hi.is_listener2_invoked);
}
