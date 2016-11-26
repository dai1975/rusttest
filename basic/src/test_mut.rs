fn muttest1() {
   let v1 = 1;
   // immutable なので破壊的操作ができない
   // v1 += 10;

   // immutable binding なので再束縛できない
   // v1 = 2;
   println!("muutest1 {:?}", v1);

   // let はできる
   let v1 = 2;

   println!("muutest1 {:?}", v1);
}

fn muttest2() {
   {
      let mut v1 = 1;
      let p1 = &1;
      println!("muutest2 *p1={}, v1={}, eq={}", *p1, v1, p1 == &v1);
      // mut なので破壊的操作ができる...が、int の場合これは別の値への束縛になるようだ。
      v1 += 10;
      println!("muutest2 *p1={}, v1={}, eq={}", *p1, v1, p1 == &v1);
   }

   {
      let mut v1 = 1;
      let p1 = &mut v1; //v1の参照を使おうとすると、v1 はアクセスできなくなる
      println!("muutest2 *p1={}, v1={}, eq={}", *p1, "borrowed", "borrowed");
      *p1 += 10;
      println!("muutest2 *p1={}, v1={}, eq={}", *p1, "borrowed", "borrowed");
   }

   {
      let mut v1 = 1;
      let w1 = 1; //もう一個 1 への let を作る
      let p1 = &mut v1;
      println!("muutest2 *p1={}, w1={}, eq={}", *p1, w1, p1 == &w1); //1,1,true
      *p1 += 10;
      println!("muutest2 *p1={}, w1={}, eq={}", *p1, w1, p1 == &w1); //11,1,false
      // primitive 型は copy-on-write ぽい挙動。
   }
}

fn muttest3() {
   let mut v0 = 0;
   // mutable ref なので破壊的操作ができる
   v0 += 10;

   // immutable binding なので再束縛できない
   //v1 = v0;

   println!("muutest3 v0={:?}", v0);
}

fn struct1() {
   // Vec::new の戻りは値
   let v0:Vec<u8> = Vec::new();
   //let v0:&Vec<u8> = Vec::new(); //error

   let v1:&Vec<u8> = &Vec::new();
   //let v0:&Vec<u8> = Vec::new(); //error

   println!("muutest4 v0={:?}, v1={:?}", v0, v1);
}

fn mutmatch1() {
   let mut value = 1;
   let &y = &value; //let はマッチングであり、双方に参照があるので let y = value に同じ。つまりコピー。
   value += 10;
   println!("mutmatch1: value={}, y={}", value, y); //yは変更されていない
}

fn mutmatch2() {
   let mut value = 1;
   let &mut mut y = &mut value; //上と同じく &mut y = &mut value のマッチング。さらに y を変更できるよう mut.
   y += 10;
   println!("mutmatch2: value={}, y={}", value, y); //yのみ変わっている
}

fn mutmatch3() {
   let mut value = 1;
   {
      let y = &mut value; //こう書けば y はmut参照である。borrow される。
      *y += 10; // deref オペレータが必要。
      println!("mutmatch3: y={}", *y); // value は borrow 取られているので使えない
   }
   println!("mutmatch3: value={}", value); //こっちも変わっている
}

fn borrow1() {
   let v0 = 5;

   {
      // immutable 変数の mutable reference を作れない
      //let v1 = &mut v0;
   }

   println!("borrow1 v0={:?}", v0);
}


fn borrow2() {
   let mut v0 = 5;

   {
      // mutable 変数の mutable reference は作れる
      let v1 = &mut v0;
      *v1 = 6;
   }

   let v2 = &v0;
   println!("borrow2 v0={:?}, v2={:?}", v0, v2);
}
fn borrow2_1() {
   let mut v0 = 5;

//   {
      // mutable 変数の mutable reference は作れる
      let v1 = &mut v0;
      *v1 = 6;
//   }
   
   // mutable reference があるのに、immutable ref を作れない
   //let v2 = &v0;
   //println!("borrow1 v0={:?}", v0); //println! も immutable ref を作る

   println!("borrow2_1 v1={:?}", v1);
}
fn borrow2_2() {
   let mut v0 = vec![1,2,3];

//   {
      // mutable 変数の mutable reference は作れる
      let v1 = &mut v0;
      v1.push(6);
//   }
   
   // mutable reference があるのに、immutable ref を作れない
   //let v2 = &v0;
   //println!("borrow1 v0={:?}", v0); //println! も immutable ref を作る
   //v0.push(1); //push も reference を作るようだ

   println!("borrow2_2 v1={:?}", v1);
}

fn ownertest1() {
   let v = vec![0,1,2]; //non Copy struct
   // push()関数は &mut を取得しようとする
   //v.push(3); //cannot borrow immutable local variable `v` as mutable
   println!("ownertest1: v={:?}", v);

   let mut w = v; //所有権移転。所有権持ってる場合は mut 付けることもできる。
   //println!("ownertest1: v={:?}", v); //error: use of moved value: `v` [E0382]
   w[0] += 10;
   println!("ownertest1: w={:?}", w);
}

fn ownertest2() {
   #[derive(Debug)]
   struct Foo {
      s: String,
   };

   let v = Foo{s:"hello".to_string()}; //non Copy struct
   //関数に付きものの借用やmoveを作らずに直アクセス(でもエラー)
   //v.s = "world".to_string(); //error: cannot assign to immutable field `v.s`
   println!("ownertest2: v={:?}", v);

   // mut付き束縛に所有権移せば書き変えできる
   let mut v = v;
   v.s = "world".to_string();
   println!("ownertest2: v={:?}", v);
}

pub fn t() {
   muttest1();
   muttest2();
   muttest3();

   struct1();

   mutmatch1();
   mutmatch2();
   mutmatch3();

   borrow1();
   borrow2();
   borrow2_1();
   borrow2_2();

   ownertest1();
   ownertest2();
}
