#[allow(dead_code)]
struct S1{ add:i32, sum:i32 }

impl S1 {
   #[allow(dead_code)]
   fn ret_ref_member(&self) -> &i32 { &self.add }

   #[allow(dead_code, unused_variables)]
   fn test_struct1(&mut self) {
      let a = self.ret_ref_member();
      //error: cannot assign to `self.sum` because it is borrowed [E0506]
      //self.sum += *a;
   }
}

struct S2 {
   count: usize,
   map: ::std::collections::HashMap<String, String>,
}
impl S2 {
   #[allow(dead_code)]
   fn mut_func(&mut self, v:&String) {
      self.count += v.len();
   }
   #[allow(dead_code)]
   fn test_struct2(&mut self, id:&String, v:&String) {
      match self.map.get(id) { // get(&self) -> Option<&T>
         //error: cannot borrow `*self` as mutable because `self.map` is also borrowed as immutable [E0502]
         None => self.mut_func(v),
         //Some(k) => (), //この k を _ などにして get() 結果が match 内に持ち越されないと mut_func は成功する
         Some(_) => (),
      }
   }
}

#[allow(dead_code)]
fn test_struct2() { //self 経由でないなら通る
   let mut waiters = ::std::collections::HashMap::<String, String>::new();
   let id   = "name".to_string();
   let name = "name".to_string();
   match waiters.get(&id) { // get(&self) -> Option<&T>
      None => {
         //ok
         waiters.insert(id.clone(), name); // insert(&mut self, T, V)
      },
      Some(_) => ()
   }
}

pub fn t() {
   println!("");println!("test_struct");
}
