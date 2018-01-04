/*
ある trait の実装を複数できるか
 */

trait TraitX {
   fn x(&self);
}
trait TraitA {
   fn a(&self);
}
trait TraitB {
   fn b(&self);
}

struct S;
impl TraitA for S {
   fn a(&self) { println!("a"); }
}
impl TraitB for S {
   fn b(&self) { println!("b"); }
}

/*
//  一つの型に同じ trait を impl しようとするとエラー。
impl TraitX for i32 {
   fn x(&self) { println!("x1"); }
}
// error[E0119]: conflicting implementations of trait `test_trait_multiimpl::TraitX` for type `i32`:
impl TraitX for i32 {
   fn x(&self) { println!("x2"); }
}
*/

/*
// 同じ型に適用されうる generics でもエラー。
// 下のコードでは A,B を実装した型は両方のジェネリクスに当てはまる。
// ダイアモンド継承ぽい関係になる。
impl <T> TraitX for T where T:TraitA {
   fn x(&self) { println!("x-a"); }
}
// error[E0119]: conflicting implementations of trait `test_trait_multiimpl::TraitX`:
impl <T> TraitX for T where T:TraitB {
   fn x(&self) { println!("x-b"); }
}
 */

fn t_a(o:&TraitA) { o.a(); }
fn t_b(o:&TraitB) { o.b(); }

#[test]
fn t() {
   let s = S;
   t_a(&s);
   t_b(&s);
}


