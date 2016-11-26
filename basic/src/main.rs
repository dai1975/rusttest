//#![feature(box_syntax, box_patterns)]

mod test_ownership;
mod test_copy;
mod test_borrow;
mod test_ref;
mod test_deref;
mod test_struct;
mod test_mut;
mod test_trait;
mod test_trait2;
mod test_tuple;
mod test_slice;
mod test_for;
mod test_mutfor;
mod test_type;
mod test_box;
mod test_string;
//mod test_graph; 色々エラー出るようになってる

fn main() {
   test_ownership::t();
   test_copy::t();
   test_borrow::t();
   test_ref::t();
   test_deref::t();
   test_mut::t();
   test_struct::t();
   test_trait::t();
   test_trait2::t();
   test_tuple::t();
   test_slice::t();
   test_for::t();
   test_mutfor::t();
   test_type::t();
   test_box::t();
   test_string::t();
//   test_graph::t();
}
