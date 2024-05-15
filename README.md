# mono event 
Blazingly fast event system

```rust
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
fn print_hmm(event: &mut SayHi) {
    println!("hmm");
}

#[test]
fn main() {
    SayHi {
        name: "Bruce".to_string(),
    }
    .dispatch()
    .unwrap();
}
```
