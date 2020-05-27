//! ```cargo
//! [dependencies]
//! rboehm = { git = "https://github.com/softdevteam/rboehm" }
//! ```
extern crate rboehm;

use rboehm::gc::Gc;
use rboehm::BoehmAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: BoehmAllocator = BoehmAllocator;

#[derive(Clone)]
struct S {
    x: usize,
}

fn main() {
    let mut x: Vec<S> = Vec::with_capacity(1);
    x.push(S { x: 123 });

    for _ in 0..5000000 {
        let _obj = Gc::new(x.clone());
    }
}
