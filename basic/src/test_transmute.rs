use std;
use super::display::ByteBuf;

fn test_u16() {
   {
      let v:u16 = 0x1234u16.to_be();
      let buf:&[u8;2] = unsafe { std::mem::transmute(&v) };
      println!("  u16be@{:?}: {:x}@{:?}", &v as *const u16, &ByteBuf(&buf[..]), buf as *const u8);
   }
   {
      let v:u16 = 0x1234u16.to_le();
      let buf:&[u8;2] = unsafe { std::mem::transmute(&v) };
      println!("  u16le@{:?}: {:x}@{:?}", &v as *const u16, &ByteBuf(&buf[..]), buf as *const u8);
   }
}
fn test_i8() {
   {
      let v:i8 = -1i8.to_be();
      let buf:&[u8;1] = unsafe { std::mem::transmute(&v) };
      println!("     i8@{:?}: {:x}@{:?}", &v as *const i8, &ByteBuf(&buf[..]), buf as *const u8);
   }
}
fn test_i16() {
   {
      let v:i16 = -2i16.to_be();
      let buf:&[u8;2] = unsafe { std::mem::transmute(&v) };
      println!("  i16be@{:?}: {:x}@{:?}", &v as *const i16, &ByteBuf(&buf[..]), buf as *const u8);
      // 00fe になる?
   }
   {
      let v:i16 = -2i16;
      let buf:&[u8;2] = unsafe { std::mem::transmute(&u16::to_be(std::mem::transmute_copy(&v))) };
      println!("  i16be@{:?}: {:x}@{:?}", &v as *const i16, &ByteBuf(&buf[..]), buf as *const u8);
   }
   {
      let v:i16 = -2i16.to_le();
      let buf:&[u8;2] = unsafe { std::mem::transmute(&v) };
      println!("  i16le@{:?}: {:x}@{:?}", &v as *const i16, &ByteBuf(&buf[..]), buf as *const u8);
   }
}

pub fn t() {
   println!("test_transmute");
   test_u16();
   test_i8();
   test_i16();
}
