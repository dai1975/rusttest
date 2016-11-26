const COMMAND_VERSION:[u8; 12]     = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];

fn from_utf8_lossy() {
   let a = COMMAND_VERSION.clone();
   let s = String::from_utf8_lossy(&a[..]);
   println!("from_utf8_lossy: {}", s); // "version     "
}
fn from_utf8_lossy2() {
   let a = COMMAND_VERSION.into_iter()
      .map(|x| *x)
      .take_while(|x| 0<*x)
      .collect::<Vec<u8>>();
   let s = String::from_utf8_lossy(&a[..]);
   println!("from_utf8_lossy: {}", s); // "version     "
}
/*
fn cstr() {
   let a = COMMAND_VERSION.clone();
   // 未実装 let s = unsafe { CStr::from_bytes_with_nul_unchecked(&a[..]) };
//   let s = CStr::from_ptr(&mem::transmute::<[u8;12],[i8;12]>(COMMAND_VERSION)[..]);
   let s = CStr::from_ptr(&mem::transmute::<[u8;12],[i8;12]>(COMMAND_VERSION)[..] as *const i8);
   println!("cstr: {}", s.to_string_lossy()); // "version     "
}
*/

pub fn t() {
   from_utf8_lossy();
   from_utf8_lossy2();
}
