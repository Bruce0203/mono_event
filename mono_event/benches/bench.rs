#![feature(min_specialization)]
#![feature(stdarch_arm_hints)]

use std::arch::aarch64::__nop;

use criterion::{criterion_group, criterion_main, Criterion};
use mono_event::{event, listen};

#[event]
struct MyEvent;

#[inline(always)]
#[no_mangle]
#[listen(MyEvent)]
fn asdf(event: &mut MyEvent) {
    unsafe {
        __nop();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| {
            let _ = MyEvent.dispatch();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
