#[cfg(test)]
mod tests {
   #[derive(Debug)]
   struct Foo {
      s: String,
   }
   impl Drop for Foo {
      fn drop(&mut self) {
         println!("  drop: {}, &self={:?}", self.s, self as *const Foo);
      }
   }

   #[test]
   fn boxtest1() {
      // スタックに確保したプリミティブ要素を box で包んだ場合にコピーは起きるか
      println!("boxtest1");
      let f = 1i32;
      let b = Box::new(f);
      println!("  boxtest1: f={:?}, &f={:?}", f, &f as *const i32);
      println!("  boxtest1: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<i32>, &*b as *const i32);
      assert!((&*b as *const i32) != (&f as *const i32)); //&f != &*b. コピーされている。アドレス的に &f はスタック、&*b はヒープ?
      let b2 = b;
      println!("  boxtest1: b={:?}, &b={:?}, &*b={:?}", b2, &b2 as *const Box<i32>, &*b2 as *const i32);
      //assert!((&*b as *const i32) != (&*b2 as *const i32)); //b は move されてる
      //b,b2 のアドレスは違うが、&*b, &*b2 は同じ。所有権コピーのコンストラクタと同じ動作。
   }

   #[test]
   fn boxtest2() {
      // スタックに確保した構造体要素を box で包んだ場合にコピーは起きるか。デストラクタはどうか。
      println!("boxtest2");
      let f = Foo { s:"helloworld".to_string() };
      println!("  boxtest2: f={:?}, &f={:?}", f, &f as *const Foo);
      let b = Box::new(f); //move
      println!("  boxtest2: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      //assert!((&*b as *const Foo) != (&f as *const Foo)); // move されたので f は使えない
      //boxtest1 に同じく、&f スタック、&*b ヒープ。
      //dropは一度だけで、&*b の方。
   }

   #[test]
   fn boxtest3() {
      // スタックに確保した box 要素を関数の戻り値として受け取る場合にコピーは起きるか
      println!("boxtest3");
      let b = boxtest3_f();
      println!("  boxtest3:  b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      // &b は変わる(コピーされる)が、&*b は boxtest3f そのまま
   }
   fn boxtest3_f() -> Box<Foo> {
      let f = Foo { s:"helloworld".to_string() };
      println!("  boxtest3f: f={:?}, &f={:?}", f, &f as *const Foo);
      let b = Box::new(f);
      println!("  boxtest3f: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      b
   }

   #[test]
   fn boxtest4() {
      // box 記法の場合に違いはあるか
      {
         println!("boxtest4");
         let f = Foo { s:"helloworld".to_string() };
         println!("  boxtest4: f={:?}, &f={:?}", f, &f as *const Foo);
         let b = box f; //move
         println!("  boxtest4: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
         //assert!((&*b as *const Foo) != (&f as *const Foo)); //move
         //boxtest1 に同じく、&f スタック、&*b ヒープ。
      }

      {
         // これなら直接ヒープ? 確認しようがない
         let b = box Foo { s:"helloworld".to_string() };
         println!("  boxtest4: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      }
   }

   #[test]
   fn boxtest5() {
      // 関数からの box 受けの場合
      println!("boxtest5");
      { // 値受け
         let f = boxtest5_f();
         println!("  boxtest5:  f={:?}, &f={:?}", f, &f as *const Foo);
         // &f は変わる(コピーされる)。drop は一度だけ。
      }
      { //stack からの box受け
         let b = box boxtest5_f();
         println!("  boxtest5:  b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
         // &f は変わる(コピーされる)。drop は一度だけ。
      }
      { //heap からの box受け
         let b = box boxtest5_f2();
         println!("  boxtest5:  b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
         // コピーされてる
         let b2 = box boxtest5_f3(b);
         println!("  boxtest5:  b={:?}, &b={:?}, &*b={:?}", b2, &b2 as *const Box<Foo>, &*b2 as *const Foo);
         // 5_f3 内は同一ポインタになっているが、b2 はコピーされてる
      }
   }
   fn boxtest5_f() -> Foo {
      let f = Foo { s:"helloworld".to_string() };
      println!("  boxtest5f: f={:?}, &f={:?}", f, &f as *const Foo);
      f
   }
   fn boxtest5_f2() -> Foo {
      let b = box boxtest5_f();
      println!("  boxtest5f2:  b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      *b
   }
   fn boxtest5_f3(b:Box<Foo>) -> Foo {
      println!("  boxtest5f3:  b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
      *b
   }
}
