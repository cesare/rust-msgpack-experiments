extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use std::fs::File;

use serde::Serialize;
use rmps::Serializer;

#[derive(Debug, PartialEq, Serialize)]
pub struct Foo {
    foo: String,
    bar: HashMap<String, i32>,
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
    foo.serialize(&mut Serializer::new(&mut file)).unwrap();
}
