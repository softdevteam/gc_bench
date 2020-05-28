//! ```cargo
//! [dependencies]
//! rboehm = { git = "https://github.com/softdevteam/rboehm" }
//! ```
extern crate rboehm;

use rboehm::gc::Gc;
use rboehm::BoehmAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: BoehmAllocator = BoehmAllocator;

fn main() {
    let mut v: Vec<usize> = Vec::with_capacity(1);
    v.push(1);

    for _ in 0..5000000 {
        let _obj = Gc::new(v.clone());
    }
}
