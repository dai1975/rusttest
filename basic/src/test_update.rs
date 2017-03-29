struct Foo {
   v:i32
}

impl Foo {
   pub fn value(&self) -> i32 { self.v }
   pub fn add(&mut self, x:i32) -> &mut Foo {
      self.v += x;
      self
   }
}

#[test]
fn test_update() {
   let mut foo = Foo { v:0 };
   foo.add(10);
   assert_eq!(foo.value(), 10);
}

#[test]
fn test_update2() {
   let mut foo = Foo { v:0 };
   foo.add(1).add(2);
   foo.add(3).add(4);
   assert_eq!(foo.value(), 10);
}
