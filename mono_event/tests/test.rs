#![feature(min_specialization)]

use mono_event::{event, highest_priority, listen, low_priority};

#[event]
pub struct SayHi;

#[highest_priority]
#[listen(SayHi)]
fn print_hi(event: &mut SayHi) {
    println!("say hi");
}

#[low_priority]
#[listen(SayHi)]
fn print_hmm(event: &mut SayHi) {
    println!("say hmm..");
}

#[test]
fn example() {
    SayHi.dispatch();
}
