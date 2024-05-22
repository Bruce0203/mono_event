#![feature(min_specialization)]

use mono_event_derive::expand_for_test;

expand_for_test!();

#[test]
fn test() {
    println!("");
    MyStruct0::MyStruct0.dispatch().unwrap();
}
