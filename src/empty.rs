extern crate serialize;
use std::io::{Writer, Reader};

pub struct Encoder<'a, W : 'a + Writer> {
    writer : &'a mut W,
}
pub struct Decoder<'a, R: 'a + Reader> {
    reader : &'a mut R,
}

macro_rules! nope {
    (fn $f:ident ( $a:ty ) -> $t:ty) => (
        fn $f(&mut self, _v: $a) -> Result<(), $t> { unimplemented!() }
    )
}

pub mod err {
    pub enum Encode {
        Other(String)
    }
    pub enum Decode {
        Other(String)
    }
}

impl<'a, W: Writer> Encoder<'a, W> {
    pub fn new<'b>(w: &'b mut W) -> Encoder<'b, W> {
        Encoder { writer: w }
    }
}

impl<'a, R: Reader> Decoder<'a, R> {
    pub fn new<'b>(r: &'b mut R) -> Decoder<'b, R> {
        Decoder { reader: r }
    }
}


impl<'a, W: Writer> serialize::Encoder<err::Encode> for Encoder<'a, W> {

    fn emit_nil(&mut self) -> Result<(), err::Encode> {
        unimplemented!();
    }
    nope!(fn emit_uint(uint) -> err::Encode)
    nope!(fn emit_u64(u64) -> err::Encode)
    nope!(fn emit_u32(u32) -> err::Encode)
    nope!(fn emit_u16(u16) -> err::Encode)
    nope!(fn emit_u8(u8) -> err::Encode)
    nope!(fn emit_int(int) -> err::Encode)
    nope!(fn emit_i64(i64) -> err::Encode)
    nope!(fn emit_i32(i32) -> err::Encode)
    nope!(fn emit_i16(i16) -> err::Encode)
    nope!(fn emit_i8(i8) -> err::Encode)
    nope!(fn emit_bool(bool) -> err::Encode)
    nope!(fn emit_f64(f64) -> err::Encode)
    nope!(fn emit_f32(f32) -> err::Encode)

    fn emit_char(&mut self, _v: char) -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_str(&mut self, _v: &str) -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_enum(&mut self, _: &str,
                 _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_enum_variant(&mut self,
                    _v_name: &str,
                    _: uint,
                    _len: uint,
                    _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                        -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_enum_variant_arg(&mut self,
                            _: uint,
                            _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                            -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_enum_struct_variant(&mut self,
                                _v_name: &str,
                                _v_id: uint,
                                _len: uint,
                                _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                               -> Result<(), err::Encode> {
        unimplemented!();
    }
    fn emit_enum_struct_variant_field(&mut self,
                                    _: &str,
                                    _: uint,
                                    _: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                                     -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_struct(&mut self,
                    _: &str,
                    _len: uint,
                    _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                  -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_struct_field(&mut self,
                        _: &str,
                        _f_idx: uint,
                        _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                        -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_tuple(&mut self,
                _len: uint,
                _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                 -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_tuple_arg(&mut self,
                    _idx: uint,
                    _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                     -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_tuple_struct(&mut self,
                        _: &str,
                        _: uint,
                        _: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                        -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_tuple_struct_arg(&mut self,
                            _: uint,
                            _: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                            -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_option(&mut self,
                    _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                  -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_option_none(&mut self) -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_option_some(&mut self,
                        _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                       -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_seq(&mut self,
                _len: uint,
                _f: |this: &mut Encoder<'a,W>| -> Result<(), err::Encode>)
               -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_seq_elt(&mut self,
                    _idx: uint,
                    _f: |this: &mut Encoder<'a,W>| -> Result<(), err::Encode>)
                   -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_map(&mut self,
                _: uint,
                _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
               -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_map_elt_key(&mut self,
                        _: uint,
                        _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                       -> Result<(), err::Encode> {
        unimplemented!()
    }
    fn emit_map_elt_val(&mut self,
                        _: uint,
                        _f: |&mut Encoder<'a,W>| -> Result<(), err::Encode>)
                       -> Result<(), err::Encode> {
        unimplemented!()
    }
}

impl<'a, R: Reader> serialize::Decoder<err::Decode> for Decoder<'a, R> {
    fn read_nil(&mut self) -> Result<(), err::Decode> {
        unimplemented!()
    }

    fn read_uint(&mut self) -> Result<uint, err::Decode> {
        unimplemented!()
    }
    fn read_u64(&mut self) -> Result<u64, err::Decode> {
        unimplemented!()
    }
    fn read_u32(&mut self) -> Result<u32, err::Decode> {
        unimplemented!()
    }
    fn read_u16(&mut self) -> Result<u16, err::Decode> {
        unimplemented!()
    }
    fn read_u8(&mut self) -> Result<u8, err::Decode> {
        unimplemented!()
    }
    fn read_int(&mut self) -> Result<int, err::Decode> {
        unimplemented!()
    }
    fn read_i64(&mut self) -> Result<i64, err::Decode> {
        unimplemented!()
    }
    fn read_i32(&mut self) -> Result<i32, err::Decode> {
        unimplemented!()
    }
    fn read_i16(&mut self) -> Result<i16, err::Decode> {
        unimplemented!()
    }
    fn read_i8(&mut self) -> Result<i8, err::Decode> {
        unimplemented!()
    }
    fn read_bool(&mut self) -> Result<bool, err::Decode> {
        unimplemented!()
    }
    fn read_f64(&mut self) -> Result<f64, err::Decode> {
        unimplemented!()
    }
    fn read_f32(&mut self) -> Result<f32, err::Decode> {
        unimplemented!()
    }
    fn read_char(&mut self) -> Result<char, err::Decode> {
        unimplemented!()
    }
    fn read_str(&mut self) -> Result<String, err::Decode> {
        unimplemented!()
    }

    // Compound types:
    fn read_enum<T>(&mut self,
                    _name: &str,
                    _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_enum_variant<T>(&mut self,
                            _names: &[&str],
                            _f: |&mut Decoder<'a,R>, uint| -> Result<T, err::Decode>)
                            -> Result<T, err::Decode> {
        unimplemented!()
    }
    fn read_enum_variant_arg<T>(&mut self,
                                _a_idx: uint,
                                _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
                                -> Result<T, err::Decode> {
        unimplemented!()
    }

    fn read_enum_struct_variant<T>(&mut self,
                                   _names: &[&str],
                                   _f: |&mut Decoder<'a,R>, uint|
                                        -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_enum_struct_variant_field<T>(&mut self,
                                         _f_name: &str,
                                         _f_idx: uint,
                                         _f: |&mut Decoder<'a,R>|
                                            -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_struct<T>(&mut self,
                    _s_name: &str,
                    _len: uint,
                    _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_struct_field<T>(&mut self,
                            _f_name: &str,
                            _f_idx: uint,
                            _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
                            -> Result<T, err::Decode> {
        unimplemented!()
    }

    fn read_tuple<T>(&mut self,
                    _len : uint,
                    _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_tuple_arg<T>(&mut self,
                        _a_idx: uint,
                        _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_tuple_struct<T>(&mut self,
                            _s_name: &str,
                            _len: uint,
                            _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_tuple_struct_arg<T>(&mut self,
                            _a_idx: uint,
                            _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    // Specialized types:
    fn read_option<T>(&mut self,
                      _f: |&mut Decoder<'a,R>, bool| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_seq<T>(&mut self,
                    _f: |&mut Decoder<'a,R>, uint| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_seq_elt<T>(&mut self,
                    _idx: uint,
                    _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn read_map<T>(&mut self,
                    _f: |&mut Decoder<'a,R>, uint| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_map_elt_key<T>(&mut self,
                        _idx: uint,
                        _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }
    fn read_map_elt_val<T>(&mut self,
                        _idx: uint,
                        _f: |&mut Decoder<'a,R>| -> Result<T, err::Decode>)
        -> Result<T, err::Decode>
    {
        unimplemented!()
    }

    fn error(&mut self, err: &str) -> err::Decode {
        return err::Decode::Other(String::from_str(err));
    }
}

