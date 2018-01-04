use std::borrow::{Borrow, Cow};

fn is_owned<'a>(x:&Cow<'a, [u8]>) -> bool {
   match x {
      &Cow::Borrowed(_) => false,
      &Cow::Owned(_) => true,
   }
}
fn get_ptr<'a>(x:&Cow<'a, [u8]>) -> *const [u8] {
   match x {
      &Cow::Borrowed(b) => b as *const [u8],
      &Cow::Owned(ref o) => o.borrow() as *const [u8],
   }
}
fn get_type<T>(_:&T) -> &'static str {
   unsafe { ::std::intrinsics::type_name::<T>() }
}
macro_rules! assert_ptr_eq (
   ($p:expr, $q:expr) => ( assert_eq!(format!("{:?}",$p), format!("{:?}",$q)) )
);
macro_rules! assert_ptr_ne (
   ($p:expr, $q:expr) => ( assert_ne!(format!("{:?}",$p), format!("{:?}",$q)) )
);

#[test]
fn test_cow_into_owned() {
   println!("cow");
   let a = [0u8; 10];
   let p = &a as *const [u8];
   let q = &a[0..3] as *const [u8];
   // p,q は位置は同じだが eq ではないらしい
   assert_ne!(p, q);
   assert_eq!(format!("{:?}",p), format!("{:?}",q));
   assert_ptr_eq!(p, q);
   {
      let c = Cow::from(&a[0..3]);
      assert_eq!(false, is_owned(&c));
      let x = get_ptr(&c);
      assert_eq!(q, x);
      assert_ptr_eq!(p, x);
   }
   {
      let c = Cow::from(&a[0..3]);
      let x = c.into_owned().borrow() as *const [u8];
      println!("  p={:?}, q={:?}, x={:?}", p, q, x);
      assert_ne!(q, x);
      assert_ptr_ne!(p, x);
   }
   {
      let c = Cow::from(&a[0..3]);
      let c = Cow::from(c.into_owned());
      let x = get_ptr(&c);
      assert_ne!(q, x);
      assert_ptr_ne!(p, x);
      let c = Cow::from(c.into_owned());
      let y = get_ptr(&c);
      println!("  p={:?}, q={:?}, x={:?}, y={:?}", p, q, x, y);
      assert_eq!(x, y);
   }
}

#[test]
fn test_cow_clone() {
   println!("cow clone");
   let a = [0u8; 10];
   let p = &a as *const [u8];
   let q = &a[0..3] as *const [u8];
   {
      let c = Cow::from(&a[0..3]);
      assert_eq!(false, is_owned(&c));
      let x = get_ptr(&c);
      assert_eq!(q, x);
      assert_ptr_eq!(p, x);
      
      let d = c.clone();
      assert_eq!(false, is_owned(&d));
      let y = get_ptr(&d);
      assert_eq!(q, y);
      assert_ptr_eq!(p, y);
      assert_eq!(x, y);
   }
}
