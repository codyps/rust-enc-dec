extern crate serialize;

macro_rules! nope {
    (fn $f:ident ( $a:ty ) -> $t:ty) => (
        fn $f(&mut self, _v: $a) -> Result<(), $t> { unimplemented!() }
    )
}

pub mod writer;
pub mod reader;

