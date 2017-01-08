// write #![feature(get_type_id)] at main.rs
use std::any::{Any};

fn test_typeid<T:Any>(prefix:&str, a:&T) {
   println!("  {}: {:?}", prefix, a.get_type_id());
}

#[allow(dead_code)]
struct Foo;

#[allow(dead_code)]
struct Bar { s: &'static str }

#[allow(dead_code)]
struct Baz<'a> { s: &'a str }

pub fn t() {
   println!("any");
   test_typeid("Foo", &Foo {});
   test_typeid("Foo", &Bar { s: "bar" });
   test_typeid("Foo", &Baz { s: "baz" });
}
