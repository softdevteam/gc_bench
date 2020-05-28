#![feature(allocator_api)]
#![feature(gc)]

use std::gc::Gc;
use std::alloc::Boehm;

#[global_allocator]
static GLOBAL_ALLOCATOR: Boehm = Boehm;

fn main() {
    let mut v: Vec<usize> = Vec::with_capacity(1);
    v.push(1);

    for _ in 0..5000000 {
        let _obj = Gc::new(v.clone());
    }
}
