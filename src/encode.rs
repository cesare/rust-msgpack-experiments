extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp;
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde::Serialize;
use rmp::Marker;
use rmp::encode::{ValueWriteError, write_map_len, write_str};
use rmps::Serializer;
use rmps::encode::VariantWriter;

#[derive(Debug, PartialEq, Serialize)]
pub struct Foo {
    foo: String,
    bar: HashMap<String, i32>,
}

struct StructMapWriter;

impl VariantWriter for StructMapWriter {
    fn write_struct_len<W: Write>(&self, wr: &mut W, len: u32) -> Result<Marker, ValueWriteError> {
        write_map_len(wr, len)
    }

    fn write_field_name<W: Write>(&self, wr: &mut W, key: &str) -> Result<(), ValueWriteError> {
        write_str(wr, key)
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert("a".to_owned(), 123);
    map.insert("b".to_owned(), 987);

    let foo = Foo {
        foo: "test".to_owned(),
        bar: map,
    };

    let mut file = File::create("foo.mp").unwrap();
    let mut serializer = Serializer::with(&mut file, StructMapWriter);
    foo.serialize(&mut serializer).unwrap();
}
