#[test]
fn test_parsenum() {
   assert_matches!(i64::from_str_radix("128", 10), Ok(128));
   assert_matches!(i64::from_str_radix("128", 16), Ok(0x128));

   //ParseIntError{kind: InvalidDigit}
   assert_matches!(i64::from_str_radix("128_", 10), Err(_));
   assert_matches!(i64::from_str_radix("_128", 10), Err(_));
   assert_matches!(u64::from_str_radix("-128", 10), Err(_));
   
   //ParseIntError{kind: Overflow}
   assert_matches!(i8::from_str_radix("128", 16), Err(_));
   assert_matches!(u8::from_str_radix("123", 16), Err(_));

   //ParseIntError{kind: Underflow}
   assert_matches!(i8::from_str_radix("-129", 10), Err(_));
}
