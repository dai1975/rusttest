#[derive(Debug)] struct X(i32);

#[allow(unused_variables)]
fn ref1() {
   let a:X   = X(39); //ownership
   let b:&X  = &a;    //reference
   let c:&X  = b;     //copy reference
   let d:&&X = &c;    //reference of refereence
}
#[allow(unused_variables)]
fn ref2() { // 左辺値に & を書くと、参照を外して束縛する
// let    &a:&X  = &X(39);  //参照がある状態で move しようとしてエラー
   let    &a:&u8 = &39u8;   //プリミティブの move セマンティクスは copy
   let     b:u8  = a;       //上で a は参照外して束縛されたので u8 型である
}
#[allow(unused_variables)]
fn ref3() { // 左辺値に ref を書くと、参照を付けて束縛する
   let ref a:&u8  = &39u8;
   let     e:&&u8 = a;     //上で ref で受けたので、一段参照が付いて &&u8 型
}

fn reffor1() {
   println!("reffor1");
   let v = vec![X(1),X(2),X(3)];
   for x in v {
      let y:X = x;
      println!("{:?}", y);
   }
   //println!("{:?}", v); //error: use of moved value: `v` [E0382]
}

fn reffor2() {
   let v = vec![X(1),X(2),X(3)];
   for x in &v {
      println!("{:?}", x);
   }
   println!("{:?}", v);
}

/*
fn mutfor1() {
   let mut v = vec![1,2,3,4,5,6,7,8,9,10];
   for x in v.iter_mut() {
      *x += 10;
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
*/
pub fn t() {
   println!("");println!("test_ref");
   ref1();
   ref2();
   ref3();
   reffor1();
   reffor2();
}
