enum Enum<'a, TT> {
   A,
   B(u32),
   N{val:u32},
   R{val:&'a [u8]},
   //T<X>{val:T},
   T{val:TT},
}
#[test]
fn test_enum2() {
   let octets = vec![0u8];
   {
      let mut v:Vec<Enum<u8>> = Vec::new();
      v.push(Enum::A);
      v.push(Enum::B(32));
      v.push(Enum::N{val:32});
      v.push(Enum::R{val:octets.as_ref()});
      v.push(Enum::T{val:255u8});
      for p in v.iter() {
         match p {
            &Enum::A => println!("A"),
            &Enum::B(ref v) => println!("B({})", v),
            &Enum::N{val:ref v} => println!("N({})", v),
            &Enum::R{val: a} => println!("R(len={}, [0]={})", a.len(), a[0]),
            &Enum::T{val: v} => println!("T({})", v),
         }
      }
   }
}

