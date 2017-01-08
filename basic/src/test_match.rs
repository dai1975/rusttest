#[cfg(test)]
mod test_match {
   struct A(i32,i32,i32,i32,i32);
   #[test]
   fn test_match() {
      println!("test_match");
      let t = (0,1,-2,3,4);
      match t {
         (0, x, 2, _, _) => println!("  t(0,{},_,_,_)",x),
         //(0, x, 2, _)    => println!("  t(0,{},_)",x), //compile error 
         (0, x, ..)      => println!("  t(0,{},...)",x),
         _ => println!("_"),
      }

      let a = A(0,1,-2,3,4);
      match a {
         A(0, x, 2, _, _) => println!("  A(0,{},_,_,_)",x),
         //A(0, x, _)       => println!("  A(0,{},_)",x), //compile error 
         A(0, x, ..)      => println!("  A(0,{},...)",x),
         _ => println!("_"),
      }
   }
}
