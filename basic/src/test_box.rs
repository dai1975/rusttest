#[derive(Debug)]
struct Foo {
   s: String,
}
impl Drop for Foo {
   fn drop(&mut self) {
      println!("drop: {}, &self={:?}", self.s, self as *const Foo);
   }
}

fn boxtest1() {
   let f = 1i32;
   let b = Box::new(f);
   println!("boxtest1: f={:?}, &f={:?}", f, &f as *const i32);
   println!("boxtest1: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<i32>, &*b as *const i32);
   //&f != &*b. コピーされている。アドレス的に &f はスタック、&*b はヒープみたい。
}

fn boxtest2() {
   let f = Foo { s:"helloworld".to_string() };
   println!("boxtest2: f={:?}, &f={:?}", f, &f as *const Foo);
   let b = Box::new(f); //move
   println!("boxtest2: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
   //boxtest1 に同じく、&f スタック、&*b ヒープ。
   //dropは一度だけで、&*b の方。
}

fn boxtest3_f() -> Box<Foo> {
   let f = Foo { s:"helloworld".to_string() };
   println!("boxtest3f: f={:?}, &f={:?}", f, &f as *const Foo);
   let b = Box::new(f);
   println!("boxtest3f:b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
   //boxtest1 に同じく、&f スタック、&*b ヒープ。
   b
}
fn boxtest3() {
   let b = boxtest3_f();
   println!("boxtest3: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
   // &*b は boxtest3f そのまま
}

fn boxtest4() { }
/* box記法はまだ使えない
fn boxtest4() {
   let f = Foo { s:"helloworld".to_string() };
   println!("boxtest4: f={:?}, &f={:?}", f, &f as *const Foo);
   let b = box f; //move
   println!("boxtest4: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
   //box 記法でも同じくコピー

   // これなら直接ヒープ? 確認しようがない
   let b = box Foo { s:"helloworld".to_string() };
   println!("boxtest4: b={:?}, &b={:?}, &*b={:?}", b, &b as *const Box<Foo>, &*b as *const Foo);
}
*/

pub fn t() {
   println!("test_box");
   boxtest1();
   boxtest2();
   boxtest3();
   boxtest4();
}
