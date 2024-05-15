#![feature(min_specialization)]

use mono_event::{event, listen};

#[event]
pub struct SayHi {
    name: String,
}

#[listen(SayHi)]
fn print_hi(event: &mut SayHi) {
    println!("hi");
}

#[listen(SayHi)]
fn print_hi(event: &mut SayHi) {
    println!("asdf");
}

#[test]
fn main() {
    SayHi {
        name: "Bruce".to_string(),
    }
    .dispatch()
    .unwrap();
}
