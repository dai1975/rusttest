#[cfg(test)]
mod test_match {
   #[derive(Debug)] struct X(i32);
   #[derive(Debug)] struct A(X,X,X,X,X);
   #[test]
   fn test_match_tuple_struct() {
      println!("test_match");

      print!("  tuple: ");
      let t = (X(0),X(1),X(-2),X(3),X(4));
      match t {
         //(X(0), x, X(2), _, _)     => println!("(0,{:?},_,_,_)",x), //move
           (X(0), ref x, X(2), _, _) => println!("(0,{:?},_,_,_)",x),
         //(0, x, 2, _)              => println!("(0,{},_)",x), //compile error. tuple項目数は一致してないとダメ。
           (X(0), ref x, ..)         => println!("(0,{:?},...)",x),
         _ => println!("_"),
      }
      println!("    {:?}", t); //move check

      print!("  struct: ");
      let a = A(X(0),X(1),X(-2),X(3),X(4));
      match a {
         //A(X(0), x, X(2), _, _) => println!("A(0,{:?},_,_,_)",x), //move
         A(X(0), ref x, X(2), _, _) => println!("A(0,{:?},_,_,_)",x),
         //A(0, x, _)       => println!("A(0,{},_)",x), //compile error. 項目数。
         A(X(0), ref x, ..)      => println!("A(0,{:?},...)",x),
         _ => println!("_"),
      }
      println!("    {:?}", a); //move check
   }

   #[test]
   fn test_match_array_slice() {
      println!("test_match");
      print!("  array: ");
      let a:[X;5] = [X(0),X(1),X(-2),X(3),X(4)];
      match a {
         //[X(0), x, X(2), _, _] => println!("[0,{:?},_,_,_]",x),  //move
         [X(0), ref x, X(2), _, _] => println!("[0,{:?},_,_,_]",x), 
         //[X(0), x, X(2), _, _, _] => //compile error. 項目数
         //[X(0), x, X(2), _]    =>
         [X(0), ref x, ..]      => println!("[0,{:?},...]",x),
         _ => println!("_"),
      }
      println!("    {:?}", a); //move check

      print!("  slice: ");
      let s:&[X] = &a[..];
      match s {
         //&[X(0), x, X(2), _, _]    => println!("&[0,{:?},_,_,_]",x), //match全体で borrow してるので、move エラー。
         &[X(0), ref x, X(2), _, _]         => println!("&[0,{:?},_,_,_]",x),
         &[X(0), ref x, X(2), _, _, _]      => println!("&[0,{:?},_,_,_,_]",x), //ok. slice では数は違っても通る
         &[X(0), ref x, X(-2), _, _, ref y] => println!("&[0,{:?},_,_,_,{:?}]",x,y), //でも不一致
         &[X(0), ref x, X(-2), ref y]       => println!("&[0,{:?},_,{:?}]",x,y),  //不足でも不一致
         &[X(0), ref x, ..]                 => println!("&[0,{:?},...]",x),
         _ => println!("_"),
      }
      println!("    {:?}", s); //move check. slice の場合は match 文でエラーになるのでここで利用しなくてもいいが。
   }      

   #[test]
   fn test_match_subslice() {
      println!("test_match_subslice");
      let a:&[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19];
      print!("  ..: ");
      match a {
         &[0,1,2, ref x.., 19] => println!("&[0,1,2,{:?},19]",x),
         _ => println!("_"),
      }
      print!("  [16]: ");
      match a {
         &[0,1,2, ref x.., 19] if x.len() == 15 => println!("&[0,1,2,[15]{:?},19]",x),
         &[0,1,2, ref x.., 19] if x.len() == 16 => println!("&[0,1,2,[16]{:?},19]",x),
         _ => println!("_"),
      }
   }      
}

