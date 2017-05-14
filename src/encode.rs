extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use std::fs::File;

use serde::Serialize;
use rmps::Serializer;

#[derive(Debug, PartialEq, Serialize)]
pub struct Foo {
    foo: String,
    bar: i32,
}

fn main() {
    let foo = Foo {
        foo: "test".to_owned(),
        bar: 123,
    };

    let mut file = File::create("foo.mp").unwrap();
    foo.serialize(&mut Serializer::new(&mut file)).unwrap();
}
