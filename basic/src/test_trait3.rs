/*
同名の関数を持つ二つの trait を実装できるか
 */

trait TraitA {
   fn t(&self);
}

trait TraitB {
   fn t(&self);
}

impl TraitA for i32 {
   fn t(&self) { println!("TraitA"); }
}
impl TraitB for i32 {
   fn t(&self) { println!("TraitB"); }
}
impl TraitB for i64 {
   fn t(&self) { println!("TraitB"); }
}

#[test]
fn t() {
   let ab:i32 = 0;
   let b:i64 = 0;

   (&ab as &TraitA).t();
   b.t();
}

