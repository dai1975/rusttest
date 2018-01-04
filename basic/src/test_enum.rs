#[allow(dead_code)]
struct Point2d { x:f32, y:f32 }
struct Point3d { x:f32, y:f32, z:f32 }
trait X {
   fn get_x(&self) -> f32;
}
impl X for Point2d {
   fn get_x(&self) -> f32 { self.x }
}
impl X for Point3d {
   fn get_x(&self) -> f32 { self.x }
}

enum Point {
   Point2d(Point2d),
   Point3d(Point3d),
}
#[test]
fn test_enum() {
   let mut v:Vec<Point> = Vec::new();
   v.push(Point::Point2d(Point2d{x:2.0, y:3.0}));
   v.push(Point::Point3d(Point3d{x:3.0, y:4.0, z:5.0}));
   for p in v.iter() {
      match p {
         &Point::Point2d(ref p) => println!("x={}, y={}", p.x, p.y),
         &Point::Point3d(ref p) => println!("x={}, y={}, z={}", p.x, p.y, p.z),
      }
   }      
}

/* trait で抽象的に扱うのは無理ぽ。
enum XPoint {
   X<T>(T),
}

#[test]
fn test_enum_trait() {
   let mut v:Vec<XPoint<X>> = Vec::new();
   v.push(XPoint::X(Point2d{x:2.0, y:3.0}));
   v.push(Point::Point3d(Point3d{x:3.0, y:4.0, z:5.0}));
   for p in v.iter() {
      match p {
         &Point::Point2d(ref p) => println!("x={}, y={}", p.x, p.y),
         &Point::Point3d(ref p) => println!("x={}, y={}, z={}", p.x, p.y, p.z),
      }
   }      
}
 */


enum PointAsX {
   Point2d(Point2d),
   Point3d(Point3d),
}

impl PointAsX {
   fn as_x(&self) -> &X {
      match *self {
         PointAsX::Point2d(ref p) => p,
         PointAsX::Point3d(ref p) => p,
      }
   }
}
/*
impl ::std::convert::AsRef<X> for PointAsX {
   fn as_ref(&self) -> &X {
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
*/

#[test]
fn test_enum_deref() {
   let mut v:Vec<PointAsX> = Vec::new();
   v.push(PointAsX::Point2d(Point2d{x:2.0, y:3.0}));
   v.push(PointAsX::Point3d(Point3d{x:3.0, y:4.0, z:5.0}));
   for p in v.iter() {
      println!("x={}", p.as_x().get_x());
   }      
}
