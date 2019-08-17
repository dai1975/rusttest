// io::Read で read すると位置は進むの?
use std::io::Read;

#[test]
fn test_read_array() {
   let orig:[u8;6] = [0,1,2,3,4,5];
   let mut tmp = [0u8;3];

   (&orig[..])  // array は Read ではないが、&[u8] は Read なので slice 化。
      .read_exact(&mut tmp);
   assert_eq!([0,1,2], tmp);  // orig の [0] から 3byte 読まれている
   assert_eq!([0,1,2,3,4,5], orig); // orig は更新されていない
}

#[test]
fn test_read_slice() {
   let orig:[u8;6] = [0,1,2,3,4,5];
   let mut tmp = [0u8;3];

   let mut slice: &[u8] = &orig[..]; //&mut [u8] ではなくて、mut &[u8] (=mut Read) なのに注意。
   slice.read_exact(&mut tmp);
   assert_eq!([0,1,2], tmp);  // orig の [0] から 3byte 読まれている
   assert_eq!([3,4,5], *slice); // slice の切り出し情報が更新されている(!)
}

#[test]
fn test_read_slice_by_ref() {
   let orig:[u8;6] = [0,1,2,3,4,5];
   let mut tmp = [0u8;3];

   let mut slice: &[u8] = &orig[..];
   {
      let s2 = slice.by_ref(); // Read#by_ref で、元 Read を参照した新たな Read を生成
      s2.read_exact(&mut tmp);
      assert_eq!([0,1,2], tmp);
      assert_eq!([3,4,5], *s2);
      assert_eq!([3,4,5], *slice); // slice は変化しない
   }
   
   assert_eq!([0,1,2], tmp);
   assert_eq!([3,4,5], slice);
}


#[test]
fn test_read_slice_by_ref_take() {
   let orig:[u8;6] = [0,1,2,3,4,5];
   let mut tmp = [0u8;3];

   let mut slice: &[u8] = &orig[..];
   {
      let s2 = slice.by_ref(); // Read#by_ref で、元 Read を参照した新たな Read を生成
      let mut s3 = s2.take(3); // take(self) で新たな Take(impl Read) を作成。self 関数なので s2 は失われる。
      s3.read_exact(&mut tmp);
      assert_eq!([0,1,2], tmp);
      assert_eq!([3,4,5], *s2); //でも使えるんだっけか。
      assert_eq!([3,4,5], *slice);
   }
   
   assert_eq!([0,1,2], tmp);
   assert_eq!([3,4,5], slice);
   // by_ref は self 取る関数使った後で元のオブジェクトを使いたい場合に使うもののようだ
}

#[test]
fn test_read_slice_copy_box() {
   let orig:[u8;6] = [0,1,2,3,4,5];
   let mut tmp = [0u8;3];

   let slice: &[u8] = &orig[..];
   {
      let mut s2 = Box::new(slice);
      s2.read_exact(&mut tmp);
      assert_eq!([0,1,2], tmp);
      assert_eq!([3,4,5], **s2); //でも使えるんだっけか。
      assert_eq!([0,1,2,3,4,5], *slice); // Box::new なら cursor=slice は新たなものになる?
   }
   
   assert_eq!([0,1,2], tmp);
   assert_eq!([0,1,2,3,4,5], slice);
   
   // でも Box::new(Read) は Read 実装によってはコピーしそうで嫌だなぁ。
}
