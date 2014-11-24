use std;
use serialize;

pub struct T<'a, R: 'a + std::io::Reader> {
    reader : &'a mut R,
}

pub enum E {
    Other(String)
}

impl<'a, R: std::io::Reader> T<'a, R> {
    pub fn new<'b>(r: &'b mut R) -> T<'b, R> {
        T { reader: r }
    }
}

impl<'a, R: std::io::Reader> serialize::Decoder<E> for T<'a, R> {
    fn read_nil(&mut self) -> Result<(), E> {
        unimplemented!()
    }

    fn read_uint(&mut self) -> Result<uint, E> {
        unimplemented!()
    }
    fn read_u64(&mut self) -> Result<u64, E> {
        unimplemented!()
    }
    fn read_u32(&mut self) -> Result<u32, E> {
        unimplemented!()
    }
    fn read_u16(&mut self) -> Result<u16, E> {
        unimplemented!()
    }
    fn read_u8(&mut self) -> Result<u8, E> {
        unimplemented!()
    }
    fn read_int(&mut self) -> Result<int, E> {
        unimplemented!()
    }
    fn read_i64(&mut self) -> Result<i64, E> {
        unimplemented!()
    }
    fn read_i32(&mut self) -> Result<i32, E> {
        unimplemented!()
    }
    fn read_i16(&mut self) -> Result<i16, E> {
        unimplemented!()
    }
    fn read_i8(&mut self) -> Result<i8, E> {
        unimplemented!()
    }
    fn read_bool(&mut self) -> Result<bool, E> {
        unimplemented!()
    }
    fn read_f64(&mut self) -> Result<f64, E> {
        unimplemented!()
    }
    fn read_f32(&mut self) -> Result<f32, E> {
        unimplemented!()
    }
    fn read_char(&mut self) -> Result<char, E> {
        unimplemented!()
    }
    fn read_str(&mut self) -> Result<String, E> {
        unimplemented!()
    }

    // Compound types:
    fn read_enum<C>(&mut self,
                    _name: &str,
                    _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_enum_variant<C>(&mut self,
                            _names: &[&str],
                            _f: |&mut T<'a,R>, uint| -> Result<C,E>)
                            -> Result<C,E> {
        unimplemented!()
    }
    fn read_enum_variant_arg<C>(&mut self,
                                _a_idx: uint,
                                _f: |&mut T<'a,R>| -> Result<C,E>)
                                -> Result<C,E> {
        unimplemented!()
    }

    fn read_enum_struct_variant<C>(&mut self,
                                   _names: &[&str],
                                   _f: |&mut T<'a,R>, uint|
                                        -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_enum_struct_variant_field<C>(&mut self,
                                         _f_name: &str,
                                         _f_idx: uint,
                                         _f: |&mut T<'a,R>|
                                            -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_struct<C>(&mut self,
                    _s_name: &str,
                    _len: uint,
                    _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_struct_field<C>(&mut self,
                            _f_name: &str,
                            _f_idx: uint,
                            _f: |&mut T<'a,R>| -> Result<C,E>)
                            -> Result<C,E> {
        unimplemented!()
    }

    fn read_tuple<C>(&mut self,
                    _len : uint,
                    _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_tuple_arg<C>(&mut self,
                        _a_idx: uint,
                        _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_tuple_struct<C>(&mut self,
                            _s_name: &str,
                            _len: uint,
                            _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_tuple_struct_arg<C>(&mut self,
                            _a_idx: uint,
                            _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    // Specialized types:
    fn read_option<C>(&mut self,
                      _f: |&mut T<'a,R>, bool| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_seq<C>(&mut self,
                    _f: |&mut T<'a,R>, uint| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_seq_elt<C>(&mut self,
                    _idx: uint,
                    _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn read_map<C>(&mut self,
                    _f: |&mut T<'a,R>, uint| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_map_elt_key<C>(&mut self,
                        _idx: uint,
                        _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }
    fn read_map_elt_val<C>(&mut self,
                        _idx: uint,
                        _f: |&mut T<'a,R>| -> Result<C,E>)
        -> Result<C,E>
    {
        unimplemented!()
    }

    fn error(&mut self, err: &str) -> E {
        return E::Other(String::from_str(err));
    }
}

