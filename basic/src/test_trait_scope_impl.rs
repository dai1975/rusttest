use super::test_trait_scope;
use super::test_trait_scope_trait;

impl test_trait_scope_trait::Trait for test_trait_scope::Foo {
   fn foo(&self) { println!("  impl") }
}
