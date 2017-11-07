trait Foo {
   const ID:&'static str = "Foo";
   fn x(&self) -> &str { Self::ID }
}
trait Bar {
   const ID:&'static str = "Bar";
   fn x(&self) -> &str { Self::ID }
}

impl Foo for i8 {
   const ID:&'static str = "Foo(i8)";
}
impl Bar for i16 {
}

#[test]
pub fn test_assoc_const_1() {
   println!("assoc_const");
   assert_eq!(i8::ID, "Foo(i8)");

   let a = 0i8;
   //error[E0038]: the trait `test_assoc_const::Foo` cannot be made into an object
   // = note: the trait cannot contain associated consts like `ID`
   //let foo:&Foo = &a as &Foo;
   //error. インスタンスから assoc const にアクセスはできないようだ。
   //println!("  a.ID = {}", a.ID);
   //ok. これならアクセスできる
   assert_eq!(a.x(), "Foo(i8)")
}

#[test]
pub fn test_assoc_const_2() {
   println!("assoc_const");
   assert_eq!(i16::ID, "Bar");

   let a = 0i16;
   assert_eq!(a.x(), "Bar")
}


