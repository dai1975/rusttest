#[derive(Debug)] //non-Copy
struct X(i16);
impl Drop for X {
   fn drop(&mut self) {
      println!(" D={:?}@{:?} [0]@{:?}", *self, self as *const X, &self.0 as *const i16);
   }
}

fn ownertest0() {
   println!("ownertest0");
   let a = X(1);
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);

//   f(a);
   let b = a;
   // メモリは a と異なるので、コピーが作られているようだ。drop は一度きり。
   // 1.13.0-beta1: メモリは a と同じになっている
   println!(" b={:?}@{:?} [0]@{:?}", b, &b as *const X, &b.0 as *const i16);
   /* error: use of moved value: `a` [E0382]
   println!(" a={:?}@{:?} [0]@{:?}", a, &a as *const X, &a.0 as *const i16);
   */
}

fn ownertest1() {
   println!("ownertest1");
   let a = X(1);
   println!(" a={:?}", a);

   let b = a;
   println!(" b={:?}", b);
   /* error: use of moved value: `a` [E0382]
   println!(" a={:?}", a);
   */
}

fn own(f:X) {
   println!(" f={:?}", f);
}
fn ownertest2() {
   println!("ownertest2");
   let a = X(1);
   println!(" a={:?}", a);

   own(a);
   /* error: use of moved value: `a` [E0382]
   println!(" a={:?}", a);
   */
}

pub fn t() {
   println!("");println!("ownertest...");
   ownertest0();
   ownertest1();
   ownertest2();
}
