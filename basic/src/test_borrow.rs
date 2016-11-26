#[derive(Debug)] //non-Copy
struct X(i16);
impl Drop for X {
   fn drop(&mut self) {
      println!(" D={:?}@{:?} [0]@{:?}", *self, self as *const X, &self.0 as *const i16);
   }
}

fn borrowtest0() {
   println!("borrowtest0");
   let a = X(1);
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);

//   f(a);
   let b = &a;
   // メモリは a と同じ
   println!(" b={:?}@{:?} [0]@{:?}", b, b as *const X, &b.0 as *const i16);
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);
}

fn borrow(f:&X) {
   println!(" f={:?}", f);
}
fn borrowtest1() {
   println!("borrowtest1");
   let a = X(1);
   println!(" a={:?}", a);

   let b = &a; // &で参照を作れば借用になる
   let c = &a; // immutable 参照は複数存在してよい
   let b2 = b; // 借用のコピーもできる
   println!(" b={:?}, c={:?}, b2={:?}", b, c, b2);
   println!(" a={:?}", a); //借用があっても所有権アクセスは可能
   borrow(&a);
   println!(" a={:?}", a);
}

fn borrowtest2() {
   println!("borrowtest2");
   let mut a = X(1);
   println!(" a={:?}", a);

   {
      let b = &mut a; // &mut
      println!(" b={:?}", b);
      //let c = &a; // error: cannot borrow `a` as immutable because it is also borrowed as mutable [E0502]
      //println!(" a={:?}", a); // println は immutable 借用呼び出しなので上と同じくエラー
   }
   let c = &a;
   println!(" c={:?}", c);
   println!(" a={:?}", a);
}

fn borrowtest3() {
   println!("borrowtest3");
   let a = X(1);
   {
      let b = &a;
      println!(" b={:?}", b);
      println!(" a={:?}", a);
      /* error: cannot move out of `a` because it is borrowed [E0505]
      let c = a; //借用がある間は所有権の移転はできない
       */
   }
   let c = a; //借用が無くなったので、所有権の移転ができる
   println!(" c={:?}", c);
}

pub fn t() {
   println!("");println!("borrowtest...");
   borrowtest0();
   borrowtest1();
   borrowtest2();
   borrowtest3();
}
