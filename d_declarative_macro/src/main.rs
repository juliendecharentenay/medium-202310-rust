
macro_rules! impl_draw {
  ( $( $p:ident ),* ) => {
    fn draw(&self) -> String {
      let points = vec![ $( self.$p, )* ];
      points.iter()
      .fold("".to_string(), 
        |r, p| if r.len() == 0 { format!("M {} {}", p.x, p.y) } else { format!("{} L {} {}", r, p.x, p.y) }
      ) + format!(" L {} {}", points[0].x, points[0].y).as_str()
    }
  };
}

#[derive(Clone, Copy)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point { Point { x, y } }
}

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

impl Square {
  impl_draw!(p1, p2, p3, p4);
}

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

impl Triangle {
  impl_draw!(p1, p2, p3);
}


fn main() {
  println!("Triangle: {}", Triangle::default().draw());
  println!("Square: {}", Square::default().draw());
}
