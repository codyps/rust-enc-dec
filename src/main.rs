#![feature(macro_rules)]
#![feature(associated_types)]
extern crate serialize;

use serialize::Encodable;
mod empty;

#[deriving(Encodable,Decodable)]
struct Foo {
    this : u64
}

fn main() {
    let mut wm = std::io::MemWriter::new();
    {
        let mut enc = empty::writer::T::new(&mut wm);
        let foo = Foo { this: 33 };
        foo.encode(&mut enc);
    }

    println!("encoded to {}", wm.unwrap())
}
