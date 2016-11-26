
trait TraitA {
   fn a(&self);
}
trait TraitB {
   fn b(&self);
}

struct A;
struct B;

impl TraitA for A {
   fn a(&self) {
      println!("a");
   }
}
impl TraitB for B {
   fn b(&self) {
      println!("b");
   }
}
// impl 対象の型を制限することはできる
impl<T:TraitA> TraitB for T {
   fn b(&self) {
      self.a();
      println!("  and b");
   }
}
// あるトレイトを実装していない、などという指定はできない。
/*
impl<T : !TraitA> TraitB for T {
   fn b(&self) {
      self.a();
      println!("  and b");
   }
}
*/

pub fn t() {
   println!("\ntrait");
   let t = A;
   t.b();

   let t = B;
   t.b();
}
