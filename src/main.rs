extern crate memmap;

use std::env;

#[inline]
fn word_boundary(c: u8) -> bool {
    c == b' ' || c == b'\r' || c == b'\t' || c == b'\n'
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("must pass exactly one argument");
    }
    let name = args.nth(1).unwrap();
    let map = memmap::Mmap::open_path(&name, memmap::Protection::Read).unwrap();
    let mut word_count = 0;
    for b in unsafe { map.as_slice() } {
        if word_boundary(*b) {
            word_count += 1;
        }
    }
    println!("{} {}", word_count, &name);
}
