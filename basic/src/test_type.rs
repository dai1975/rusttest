/* 特殊化はできないので、なんとか動的にできないか
fn <T> foo(v: &T)  -> { print!("generics"); }
fn     foo(v: &u8) -> { print!("specific to u8"); }
*/

// primitive 型は Any 実装してなさそうだが、しているようだ。
use std::any::Any;
fn foo<T:Any>(v: &T) {
   let any = v as &Any;
   match any.downcast_ref::<u8>() {
      Some(as_u8) => {
         println!("u8: {}", as_u8);
      },
      None => {
         println!("generic");
      },
   }
}

pub fn t() {
   println!("\ntype");
   foo(&"abc".to_string());
   foo(&1u8);
}
