
trait Trait {
   fn a<T>(&self, v:&T);
}

struct A;

trait StringTrait { }
impl StringTrait for String { }

trait IntTrait { }
impl IntTrait for i32 { }

/*
impl Trait for A {
   //構造体を指定して特殊化はできない
   fn a<T:String>(&self, v:&T) { println!("String"); }
   fn a<>(&self, v:&String) { println!("String"); }    //C++風

   // よくわからんエラー
   //error: the requirement `T : test_trait2::StringTrait` appears on the impl method but not on the corresponding trait method [E0276]
   fn a<T:StringTrait>(&self, v:&T) {
      println!("String");
   }

   // 型によって動作を変える(オーバーロード)ことはできない?
   fn a<T:IntTrait>(&self, v:&T) {
      println!("int");
   }
}
*/

impl IntTrait for A { }
impl <T:IntTrait> Trait for T {
   fn a<U>(&self, _v:&U) {
      println!("int");
   }
}
/* 同じトレイトを実装できない?
impl StringTrait for A { }
impl <T:StringTrait> Trait for T {
   fn a<U>(&self, v:&U) {
      println!("String");
   }
}
*/

pub fn t() {
   println!("\ntrait2");
   let t = A;
   t.a(&10);
   //String は実装できてない
   //t.a("abc");
}
