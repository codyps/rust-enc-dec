#![feature(macro_rules)]
#![feature(associated_types)]
extern crate serialize;

mod empty;

#[deriving(Encodable,Decodable)]
struct Foo {
    this : u64
}

fn main() {
    let wm = std::io::MemWriter::new();
    let enc = empty::Encoder::new(wm);

    println!("Hello, world!")
}
