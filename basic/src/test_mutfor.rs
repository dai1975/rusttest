fn mutfor1() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];
   for x in v.iter_mut() {
      let y:&mut i32 = x;
      *y += 10;
   }
   println!("mutfor1: {:?}", v);
}

fn mutfor2() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];
   for x in &mut v { // in に書いた &mut は iter_mut と同じ(シンタックスシュガー?)
      *x += 10;
   }
   println!("mutfor2: {:?}", v);
}

fn mutfor3() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];
   for &mut mut x in &mut v {
      x += 10; // "let &mut x = &mut y" はマッチングらしい。x は y のコピーになる。
      println!("{}", x); // x はコピーなので変更しても意味はない
   }
   println!("mutfor3: {:?}", v);
}

pub fn t() {
   mutfor1();
   mutfor2();
   mutfor3();
}
