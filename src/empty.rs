extern crate serialize;
use std::io::{Writer, Reader};

pub struct Encoder<W : Writer> {
    writer : W,
}
pub struct Decoder<R: Reader> {
    reader : R,
}

macro_rules! nope {
    (fn $f:ident ( $a:ty ) -> $t:ty) => (
        fn $f(&mut self, _v: $a) -> Result<(), $t> { unimplemented!() }
    )
}

pub mod encode {
    pub enum Error {
        Other(String)
    }
}

pub mod decode {
    pub enum Error {
        Other(String)
    }
}

impl<W: Writer> Encoder<W> {
    pub fn new(w: W) -> Encoder<W> {
        Encoder { writer: w }
    }
}

impl<R: Reader> Decoder<R> {
    pub fn new<R: Reader>(r: R) -> Decoder<R> {
        Decoder { reader: r }
    }
}


impl<W: Writer> serialize::Encoder<encode::Error> for Encoder<W> {

    fn emit_nil(&mut self) -> Result<(), encode::Error> {
        unimplemented!();
    }
    nope!(fn emit_uint(uint) -> encode::Error)
    nope!(fn emit_u64(u64) -> encode::Error)
    nope!(fn emit_u32(u32) -> encode::Error)
    nope!(fn emit_u16(u16) -> encode::Error)
    nope!(fn emit_u8(u8) -> encode::Error)
    nope!(fn emit_int(int) -> encode::Error)
    nope!(fn emit_i64(i64) -> encode::Error)
    nope!(fn emit_i32(i32) -> encode::Error)
    nope!(fn emit_i16(i16) -> encode::Error)
    nope!(fn emit_i8(i8) -> encode::Error)
    nope!(fn emit_bool(bool) -> encode::Error)
    nope!(fn emit_f64(f64) -> encode::Error)
    nope!(fn emit_f32(f32) -> encode::Error)

    fn emit_char(&mut self, _v: char) -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_str(&mut self, _v: &str) -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_enum(&mut self, _: &str,
                 _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_enum_variant(&mut self,
                    _v_name: &str,
                    _: uint,
                    _len: uint,
                    _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                        -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_enum_variant_arg(&mut self,
                            _: uint,
                            _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                            -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_enum_struct_variant(&mut self,
                                _v_name: &str,
                                _v_id: uint,
                                _len: uint,
                                _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                               -> Result<(), encode::Error> {
        unimplemented!();
    }
    fn emit_enum_struct_variant_field(&mut self,
                                    _: &str,
                                    _: uint,
                                    _: |&mut Encoder<W>| -> Result<(), encode::Error>)
                                     -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_struct(&mut self,
                    _: &str,
                    _len: uint,
                    _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                  -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_struct_field(&mut self,
                        _: &str,
                        _f_idx: uint,
                        _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                        -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_tuple(&mut self,
                _len: uint,
                _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                 -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_tuple_arg(&mut self,
                    _idx: uint,
                    _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                     -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_tuple_struct(&mut self,
                        _: &str,
                        _: uint,
                        _: |&mut Encoder<W>| -> Result<(), encode::Error>)
                        -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_tuple_struct_arg(&mut self,
                            _: uint,
                            _: |&mut Encoder<W>| -> Result<(), encode::Error>)
                            -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_option(&mut self,
                    _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                  -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_option_none(&mut self) -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_option_some(&mut self,
                        _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                       -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_seq(&mut self,
                _len: uint,
                _f: |this: &mut Encoder<W>| -> Result<(), encode::Error>)
               -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_seq_elt(&mut self,
                    _idx: uint,
                    _f: |this: &mut Encoder<W>| -> Result<(), encode::Error>)
                   -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_map(&mut self,
                _: uint,
                _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
               -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_map_elt_key(&mut self,
                        _: uint,
                        _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                       -> Result<(), encode::Error> {
        unimplemented!()
    }
    fn emit_map_elt_val(&mut self,
                        _: uint,
                        _f: |&mut Encoder<W>| -> Result<(), encode::Error>)
                       -> Result<(), encode::Error> {
        unimplemented!()
    }
}

impl<R: Reader> serialize::Decoder<decode::Error> for Decoder<R> {
    fn read_nil(&mut self) -> Result<(), decode::Error> {
        unimplemented!()
    }

    fn read_uint(&mut self) -> Result<uint, decode::Error> {
        unimplemented!()
    }
    fn read_u64(&mut self) -> Result<u64, decode::Error> {
        unimplemented!()
    }
    fn read_u32(&mut self) -> Result<u32, decode::Error> {
        unimplemented!()
    }
    fn read_u16(&mut self) -> Result<u16, decode::Error> {
        unimplemented!()
    }
    fn read_u8(&mut self) -> Result<u8, decode::Error> {
        unimplemented!()
    }
    fn read_int(&mut self) -> Result<int, decode::Error> {
        unimplemented!()
    }
    fn read_i64(&mut self) -> Result<i64, decode::Error> {
        unimplemented!()
    }
    fn read_i32(&mut self) -> Result<i32, decode::Error> {
        unimplemented!()
    }
    fn read_i16(&mut self) -> Result<i16, decode::Error> {
        unimplemented!()
    }
    fn read_i8(&mut self) -> Result<i8, decode::Error> {
        unimplemented!()
    }
    fn read_bool(&mut self) -> Result<bool, decode::Error> {
        unimplemented!()
    }
    fn read_f64(&mut self) -> Result<f64, decode::Error> {
        unimplemented!()
    }
    fn read_f32(&mut self) -> Result<f32, decode::Error> {
        unimplemented!()
    }
    fn read_char(&mut self) -> Result<char, decode::Error> {
        unimplemented!()
    }
    fn read_str(&mut self) -> Result<String, decode::Error> {
        unimplemented!()
    }

    // Compound types:
    fn read_enum<T>(&mut self,
                    _name: &str,
                    _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_enum_variant<T>(&mut self,
                            _names: &[&str],
                            _f: |&mut Decoder<R>, uint| -> Result<T, decode::Error>)
                            -> Result<T, decode::Error> {
        unimplemented!()
    }
    fn read_enum_variant_arg<T>(&mut self,
                                _a_idx: uint,
                                _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
                                -> Result<T, decode::Error> {
        unimplemented!()
    }

    fn read_enum_struct_variant<T>(&mut self,
                                   _names: &[&str],
                                   _f: |&mut Decoder<R>, uint|
                                        -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_enum_struct_variant_field<T>(&mut self,
                                         _f_name: &str,
                                         _f_idx: uint,
                                         _f: |&mut Decoder<R>|
                                            -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_struct<T>(&mut self,
                    _s_name: &str,
                    _len: uint,
                    _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_struct_field<T>(&mut self,
                            _f_name: &str,
                            _f_idx: uint,
                            _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
                            -> Result<T, decode::Error> {
        unimplemented!()
    }

    fn read_tuple<T>(&mut self,
                    _len : uint,
                    _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_tuple_arg<T>(&mut self,
                        _a_idx: uint,
                        _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_tuple_struct<T>(&mut self,
                            _s_name: &str,
                            _len: uint,
                            _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_tuple_struct_arg<T>(&mut self,
                            _a_idx: uint,
                            _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    // Specialized types:
    fn read_option<T>(&mut self,
                      _f: |&mut Decoder<R>, bool| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_seq<T>(&mut self,
                    _f: |&mut Decoder<R>, uint| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_seq_elt<T>(&mut self,
                    _idx: uint,
                    _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn read_map<T>(&mut self,
                    _f: |&mut Decoder<R>, uint| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_map_elt_key<T>(&mut self,
                        _idx: uint,
                        _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }
    fn read_map_elt_val<T>(&mut self,
                        _idx: uint,
                        _f: |&mut Decoder<R>| -> Result<T, decode::Error>)
        -> Result<T, decode::Error>
    {
        unimplemented!()
    }

    fn error(&mut self, err: &str) -> decode::Error {
        return decode::Other(String::from_str(err));
    }
}

