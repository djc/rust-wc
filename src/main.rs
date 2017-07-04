extern crate memmap;

use std::env;

#[inline]
fn word_boundary(c: u8) -> bool {
    c == b'\r' || c == b'\t' || c == b' '
}

#[inline]
fn line_boundary(c: u8) -> bool {
    c == b'\n'
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("must pass exactly one argument");
    }
    let name = args.nth(1).unwrap();
    let map = memmap::Mmap::open_path(&name, memmap::Protection::Read).unwrap();
    let mut byte_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;
    for b in unsafe { map.as_slice() } {
        byte_count += 1;
        if line_boundary(*b) {
            line_count += 1;
        } else if word_boundary(*b) {
            word_count += 1;
        }
    }
    println!("{} {} {} {}", line_count, word_count, byte_count, &name);
}
