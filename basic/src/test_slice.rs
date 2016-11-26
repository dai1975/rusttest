fn vec_slice1() {
   let v = vec![1,2,3,4,5,6,7,8,9,10];
   let s = &v[3..7];
   println!("vec_slice1: {:?}", s); //[4,5,6,7]
}

fn vec_slice2() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];
   {
      let s = &mut v[3..7];
      s[0] = 99;
      println!("vec_slice2: {:?}", s); //[99,5,6,7]
   }
   println!("vec_slice2: v1={:?}", v); //[1,2,3,99,5,6,7,8,9,10]
}

fn vec_slice3() {
   let mut v:Vec<u8> = Vec::with_capacity(10);
   for i in 0..5 { v.push(i); }
   println!("vec_slice3: len={}, cap={}", v.len(), v.capacity());
   {
      let s = &v[..];
      println!("vec_slice3: &[..].len()={}", s.len());
   }
   {
      let s = &mut v[..];
      println!("vec_slice3: &mut [..].len()={}", s.len());
      //s[5] = 5; //error
      //println!("vec_slice3: &mut [..].len()={}", s.len());
   }
}

fn vec_slice4() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];

   {
      let (left,right) = v.split_at_mut(5);
      println!("vec_slice4: l={:?}, r={:?}", left, right);

      right[0] = 99;
      println!("vec_slice4: l={:?}, r={:?}", left, right);
   }
   println!("vec_slice4: v={:?}", v);

   {
      let (left,right) = v.split_at_mut(5);
      right.clone_from_slice(left);
      println!("vec_slice4: l={:?}, r={:?}", left, right);
   }
   println!("vec_slice4: v={:?}", v);
}

pub fn t() {
   vec_slice1();
   vec_slice2();
   vec_slice3();
   vec_slice4();
}
