use std;
use serialize;

pub enum E {
    Other(String)
}

/// An Encoder which compiles but always returns an error
pub struct T<'a, W : 'a + std::io::Writer> {
    writer : &'a mut W,
}

impl<'a, W : 'a + std::io::Writer> T<'a, W> {
    pub fn new<'b>(w: &'b mut W) -> T<'b, W> {
        T { writer: w }
    }
}

impl<'a, W: std::io::Writer> serialize::Encoder<E> for T<'a, W> {

    fn emit_nil(&mut self) -> Result<(), E> {
        unimplemented!();
    }
    nope!(fn emit_uint(uint) -> E)
    nope!(fn emit_u64(u64) -> E)
    nope!(fn emit_u32(u32) -> E)
    nope!(fn emit_u16(u16) -> E)
    nope!(fn emit_u8(u8) -> E)
    nope!(fn emit_int(int) -> E)
    nope!(fn emit_i64(i64) -> E)
    nope!(fn emit_i32(i32) -> E)
    nope!(fn emit_i16(i16) -> E)
    nope!(fn emit_i8(i8) -> E)
    nope!(fn emit_bool(bool) -> E)
    nope!(fn emit_f64(f64) -> E)
    nope!(fn emit_f32(f32) -> E)

    fn emit_char(&mut self, _v: char) -> Result<(), E> {
        unimplemented!();
    }
    fn emit_str(&mut self, _v: &str) -> Result<(), E> {
        unimplemented!();
    }
    fn emit_enum(&mut self, _: &str,
                 _f: |&mut T<'a,W>| -> Result<(), E>)
                -> Result<(), E> {
        unimplemented!();
    }
    fn emit_enum_variant(&mut self,
                    _v_name: &str,
                    _: uint,
                    _len: uint,
                    _f: |&mut T<'a,W>| -> Result<(), E>)
                        -> Result<(), E> {
        unimplemented!();
    }
    fn emit_enum_variant_arg(&mut self,
                            _: uint,
                            _f: |&mut T<'a,W>| -> Result<(), E>)
                            -> Result<(), E> {
        unimplemented!();
    }
    fn emit_enum_struct_variant(&mut self,
                                _v_name: &str,
                                _v_id: uint,
                                _len: uint,
                                _f: |&mut T<'a,W>| -> Result<(), E>)
                               -> Result<(), E> {
        unimplemented!();
    }
    fn emit_enum_struct_variant_field(&mut self,
                                    _: &str,
                                    _: uint,
                                    _: |&mut T<'a,W>| -> Result<(), E>)
                                     -> Result<(), E> {
        unimplemented!()
    }
    fn emit_struct(&mut self,
                    _: &str,
                    _len: uint,
                    _f: |&mut T<'a,W>| -> Result<(), E>)
                  -> Result<(), E> {
        unimplemented!()
    }
    fn emit_struct_field(&mut self,
                        _: &str,
                        _f_idx: uint,
                        _f: |&mut T<'a,W>| -> Result<(), E>)
                        -> Result<(), E> {
        unimplemented!()
    }
    fn emit_tuple(&mut self,
                _len: uint,
                _f: |&mut T<'a,W>| -> Result<(), E>)
                 -> Result<(), E> {
        unimplemented!()
    }
    fn emit_tuple_arg(&mut self,
                    _idx: uint,
                    _f: |&mut T<'a,W>| -> Result<(), E>)
                     -> Result<(), E> {
        unimplemented!()
    }
    fn emit_tuple_struct(&mut self,
                        _: &str,
                        _: uint,
                        _: |&mut T<'a,W>| -> Result<(), E>)
                        -> Result<(), E> {
        unimplemented!()
    }
    fn emit_tuple_struct_arg(&mut self,
                            _: uint,
                            _: |&mut T<'a,W>| -> Result<(), E>)
                            -> Result<(), E> {
        unimplemented!()
    }
    fn emit_option(&mut self,
                    _f: |&mut T<'a,W>| -> Result<(), E>)
                  -> Result<(), E> {
        unimplemented!()
    }
    fn emit_option_none(&mut self) -> Result<(), E> {
        unimplemented!()
    }
    fn emit_option_some(&mut self,
                        _f: |&mut T<'a,W>| -> Result<(), E>)
                       -> Result<(), E> {
        unimplemented!()
    }
    fn emit_seq(&mut self,
                _len: uint,
                _f: |this: &mut T<'a,W>| -> Result<(), E>)
               -> Result<(), E> {
        unimplemented!()
    }
    fn emit_seq_elt(&mut self,
                    _idx: uint,
                    _f: |this: &mut T<'a,W>| -> Result<(), E>)
                   -> Result<(), E> {
        unimplemented!()
    }
    fn emit_map(&mut self,
                _: uint,
                _f: |&mut T<'a,W>| -> Result<(), E>)
               -> Result<(), E> {
        unimplemented!()
    }
    fn emit_map_elt_key(&mut self,
                        _: uint,
                        _f: |&mut T<'a,W>| -> Result<(), E>)
                       -> Result<(), E> {
        unimplemented!()
    }
    fn emit_map_elt_val(&mut self,
                        _: uint,
                        _f: |&mut T<'a,W>| -> Result<(), E>)
                       -> Result<(), E> {
        unimplemented!()
    }
}

