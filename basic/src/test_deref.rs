#[derive(Debug)] struct X(i32);
#[derive(Debug)] struct Y(X);
impl ::std::ops::Deref for Y {
   type Target = X;
   fn deref(&self) -> &Self::Target { &self.0 }
}
impl X {
   fn foo(&self) { println!("  X.foo({})", self.0); }
}

fn test_impl_deref() {
   println!("test_impl_deref");
   let a = Y(X(39));
   (*a).foo();
   a.foo(); //暗黙のderef
}

fn test_ref_deref() {
   println!("test_ref_deref");
   let a:Vec<u8>  = Vec::new();                                                                                         
   let b:&Vec<u8> = &a;
   println!("  a.cap={}",      a.capacity());
   println!("  (*b).cap()={}", (*b).capacity());
   // &Vec に capacity() は無いが、自動的に deref される
   println!("  b.cap={}",      b.capacity());                                                                                    
}
fn test_vec_deref() { //Vec<T> の deref は &[T](=スライス) 
   println!("test_vec_deref");
   let a:Vec<u8> = Vec::new();
   // error: no method named `deref` found for type `std::vec::Vec<u8>` in the current scope
   //println!("  vec.deref.into_vec={:?}", a.deref().into_vec());
   println!("  (*vec).to_vec={:?}",    (*a).to_vec());
   println!("  vec.to_vec={:?}",       a.to_vec());

   use ::std::ops::Deref;
   println!("  vec.deref.to_vec={:?}", a.deref().to_vec());
   println!("  vec.to_vec={:?}",       a.to_vec());                                     
}


pub fn t() {
   println!("");println!("test_deref");
   test_impl_deref();
   test_ref_deref();
   test_vec_deref();
}
