#![feature(get_type_id)]
#![feature(associated_consts)]
#![feature(slice_patterns, advanced_slice_patterns)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]
#[macro_use] extern crate assert_matches;

mod display;

mod test_ownership;
mod test_copy;
mod test_borrow;
mod test_ref;
mod test_deref;
mod test_struct;
mod test_mut;
mod test_transmute;
mod test_trait;
mod test_trait2;
mod test_trait3;
mod test_tuple;
mod test_slice;
mod test_for;
mod test_mutfor;
mod test_type;
mod test_box;
mod test_string;
//mod test_graph; 色々エラー出るようになってる
mod test_eq;
mod test_any;
mod test_assoc_const;
mod test_trait_scope;
mod test_trait_scope_trait;
mod test_trait_scope_impl;
mod test_trait_multiimpl;
mod test_match;
mod test_ret_refcell;
mod test_update;
mod test_cursor;
mod test_self_implref;
mod test_enum;
mod test_enum2;
mod test_borrowtrait;
mod test_cow;
mod test_index;
mod test_parsenum;

fn main() {
   test_ownership::t();
   test_copy::t();
   test_borrow::t();
   test_ref::t();
   test_deref::t();
   test_mut::t();
   test_transmute::t();
   test_struct::t();
   test_trait::t();
   test_trait2::t();
   test_tuple::t();
   test_slice::t();
   test_for::t();
   test_mutfor::t();
   test_type::t();
   test_string::t();
   //   test_graph::t();
   test_eq::t();
   test_any::t();
   test_trait_scope::t();
}
