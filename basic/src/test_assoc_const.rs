trait Foo {
   const ID:i32 = 1;
   fn x(&self) -> i32 { Self::ID }
}

impl Foo for i8 {
}
impl Foo for i16 {
   const ID:i32 = 2;
}

pub fn t() {
   println!("assoc_const");
   println!("   i8::ID = {}", i8::ID);
   println!("  i16::ID = {}", i16::ID);

   let a8  = 0i8;
   let a16 = 0i16;
   #[allow(unused_variables)]
   let foo:&Foo = &a8;
   //error. インスタンスから assoc const にアクセスはできないようだ。
   //println!("   a8::ID = {}", a8.ID, a8::ID);
   //println!("  foo::ID = {}", foo.ID, foo::ID);
   //ok. これならアクセスできる
   println!("    a8.x() = {}", a8.x());
   println!("   a16.x() = {}", a16.x());
}


