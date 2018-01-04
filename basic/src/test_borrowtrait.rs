use ::std::borrow::Borrow;

#[derive(Default)]
struct BorrowHolder<'a> {
   pub holder: Vec<&'a [u8]> //Borrowは Sized ではないので、Vec のメンバ型にはできない
}

impl<'a> BorrowHolder<'a> {
   fn push_lifetime<'x>(&mut self, v: &'x [u8]) {
      //self.holder.push(v); //E0312: lifetime of reference outlives lifetime of borrowed content
      //self.holder.push(v.borrow()); //v.borrow() は lifetime 無し、つまりスタック上のメモリを返す
   }
   fn push(&mut self, v: &'a [u8]) {
      self.holder.push(v);
      self.holder.push(v.borrow());
   }
   fn dump(&self) {
      for i in 0..self.holder.len() {
         //println!("{}: {:?}", i, &self.holder[i] as *const Borrow<[u8]>);
         println!("{}: {:?}", i, self.holder[i] as *const [u8]);
      }
   }
}

#[test]
fn test_borrowtrait_lifetime() {
   {
      let a:[u8;10] = [0,1,2,3,4,5,6,7,8,9];
      let pa = &a[0..10];
      let p = &a as *const [u8];
      let mut h = BorrowHolder::default();
      println!("&a: {:?}", &a as *const [u8]);
      println!("pa: {:?}", pa as *const [u8]); // &a と同じ
      assert_eq!(p, pa as *const [u8]);
      h.push(pa);
      h.dump();
      assert_eq!(p, h.holder[0] as *const [u8]);
      assert_eq!(h.holder[0] as *const [u8], h.holder[1] as *const [u8]);
   }
}
