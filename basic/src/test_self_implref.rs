pub trait T: Sized { // foo(self) 実装には Sized が必要のようだ
   fn foo(self) {
      println!("foo");
   }
}

#[test]
fn test_impl_val() {
   struct A();
   impl T for A { }
   {
      let a = A();
      a.foo();
      // a.foo(); これはエラー。
   }
   {
      let a = &A();
      // a.foo(); borrow を消費しようとしてエラー。
   }
}

#[test]
fn test_impl_ref() {
   struct A();
   impl <'a> T for &'a A { } // for &A には lifetime 必須のようだ
   {
      let a = A();
      a.foo();
      a.foo(); //これは通る(!)
   }
   {
      let a = &A();
      a.foo();
      a.foo();
   }
}

#[test]
fn test_impl_mutref() {
   struct A();
   impl <'a> T for &'a mut A { }
   {
      let mut a = A(); //mut val じゃないと mut borrow は取れない
      a.foo();
      a.foo();
   }
   {
      let a = &mut A(); //最初から mut ref ならもちろん問題ない
      a.foo();
      a.foo();
   }
}


