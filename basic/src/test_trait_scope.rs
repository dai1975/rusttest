pub struct Foo { }

pub fn t() {
   println!("test_trait_scope");
   let f = Foo { };
   /* 失敗
   f.foo();
    */

   /* 当然失敗
   use super::test_trait_scope_trait;
   f.foo();
    */

   // 成功。実装は use しなくてもよい(main.js で mod した時点でリンクされて見えるのだろう)。
   use super::test_trait_scope_trait::Trait;
   f.foo();
}
