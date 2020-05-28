#![feature(allocator_api)]
#![feature(gc)]

use std::gc::Gc;
use std::alloc::Boehm;
use std::gc::ManageableContents;

#[global_allocator]
static GLOBAL_ALLOCATOR: Boehm = Boehm;

#[derive(Clone)]
struct S {
    x: usize,
}

impl ManageableContents for S {}

fn main() {
    let mut x: Vec<S> = Vec::with_capacity(1);
    x.push(S { x: 123 });

    for _ in 0..5000000 {
        let _obj = Gc::new(x.clone());
    }
}
