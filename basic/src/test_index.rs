#[test]
#[should_panic]
fn test_vec_outofrange() {
   let v = vec![0,1,2];
   v[100usize];
}
