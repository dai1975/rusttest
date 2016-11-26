#[derive(Debug,Copy,Clone)]
struct X(i16);

fn copytest0() {
   println!("copytest0");
   let a = X(1);
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);

//   f(a);
   let b = a;
   // メモリは a と異なるので、コピーが作られているようだ。drop は一度きり。
   println!(" b={:?}@{:?} [0]@{:?}", b, &b as *const X, &b.0 as *const i16);
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);
}

fn copytest1() {
   println!("copytest1");
   let a = X(1);
   println!(" a={:?}", a);

   let b = a;
   println!(" b={:?}", b);
   println!(" a={:?}", a);
}

fn own(f:X) {
   println!(" f={:?}", f);
}
fn copytest2() {
   println!("copytest2");
   let a = X(1);
   println!(" a={:?}", a);

   own(a);
   println!(" a={:?}", a);
}

pub fn t() {
   println!("");
   println!("copytest...");
   copytest0();
   copytest1();
   copytest2();
}
