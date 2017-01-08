fn t_nan() {
   let a = 0.0f32/0.0f32;
   let b = a * 2.0f32;
   println!("nan={}, infinit={}, eq={}", a.is_nan(), a.is_infinite(), a==b);
}
fn t_infinite() {
   let a = 1.0f32/0.0f32;
   let b = a * 2.0f32;
   println!("nan={}, infinit={}, eq={}", a.is_nan(), a.is_infinite(), a==b);
}

pub fn t() {
   println!("eq test");
   t_nan();
   t_infinite();
}
