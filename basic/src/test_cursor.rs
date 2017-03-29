#[test]
fn test_cursor_vec() {
   use std::io::Write;
   let mut v:Vec<u8> = vec![0;100];
   assert_eq!(v.write(&[1,2,3]).unwrap(), 3);
   assert_eq!(v.len(), 103);
   assert_eq!(v.write(&[4,5,6]).unwrap(), 3);
   assert_eq!(v.len(), 106);
   assert_eq!(&v[0..6], &[0,0,0,0,0,0]);
   assert_eq!(&v[100..], &[1,2,3,4,5,6]);
}
