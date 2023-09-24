#[derive(Clone, Copy)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point { Point { x, y } }
}

#[derive(e2_derive_draw::DeriveDraw)]
pub struct Square { 
  p1: Point,
  p2: Point,
  p3: Point,
  p4: Point,
}

impl Default for Square {
  fn default() -> Square {
    Square {
      p1: Point::new(0.0, 0.0),
      p2: Point::new(1.0, 0.0),
      p3: Point::new(1.0, 1.0),
      p4: Point::new(0.0, 1.0),
    }
  }
}

#[derive(e2_derive_draw::DeriveDraw)]
pub struct Triangle { 
  p1: Point,
  p2: Point,
  p3: Point,
}

impl Default for Triangle {
  fn default() -> Triangle {
    Triangle {
      p1: Point::new(0.0, 0.0),
      p2: Point::new(1.0, 0.0),
      p3: Point::new(0.0, 1.0),
    }
  }
}

fn main() {
  println!("Triangle: {}", Triangle::default().draw());
  println!("Square: {}", Square::default().draw());
}
