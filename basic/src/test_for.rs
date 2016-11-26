struct X(i32,i32,i32);

#[allow(unused_variables)]
fn test_for1() {
   let a = X(100,101,102);
   //error: the trait bound `test_for::X: std::iter::Iterator` is not satisfied [E0277]
   //for x in a { println!("{}", x); }
}

pub fn t() {
   println!("");println!("test_for");
   test_for1();
}


/*

for は std::iter::Iterator トレイトを受けとる。

std::iter::IntoIterator<T> トレイトを実装している場合、
  type IntoIter: Iterator;
  fn into_iter(self) -> Self::IntoIter
を呼んで、戻り値の Iterator に対して処理する。
この into_iter は self を move で受けとることに注意。

Vec<T> は IntoIterator を実装している。型は std::vec::IntoIter<T> である。
IntoIter<T> は T を返すイテレータ実装。
  for x in v { ... }
としたとき、v の所有権を失い、x には v の中身が move される。

&Vec も IntoIterator を実装している。型は std::slice::Iter<T> である。
Iter<T>  は &T を返すイテレータ実装。
  for x in &v { ... }
としたとき、&v の所有権を失い(つまり v の借用を失う)、x には v の中身の参照が渡される。
&mut Vec も同様。

Vec<T> は Deref,DerefMut を実装していて、変換対象の型はそれぞれ &[T], &mut [T] である。
&[T] は IntoIterator を実装しているが、Vec の for では関係ないのでおいておく。
&[T] は関数
   fn iter(&self)         -> std::slice::Iter<T>
   fn iter_mut(&mut self) -> std::slice::IterMut<T>
を実装している。Iter<T> が next で返す型は上で見たように &T である。
  for x in v.iter() { ... }
としたとき、暗黙の deref(&self) によって &[T] に変換されて iter(&self) が呼ばれる。
結果的に、for x in v.iter() では v の借用が渡され、 x には v の内部要素の参照が渡される。
つまり、
  for x in &v { ... }
と同等である。

 */
