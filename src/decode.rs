extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use std::fs::File;

use serde::Deserialize;
use rmps::Deserializer;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Foo {
    foo: String,
    bar: i32,
}

fn main() {
    let file = File::open("foo.mp").unwrap();
    let mut deserializer = Deserializer::new(&file);

    let results: Foo = Deserialize::deserialize(&mut deserializer).unwrap();
    println!("{:?}", results);
}
